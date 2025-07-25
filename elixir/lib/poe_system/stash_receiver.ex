defmodule PoeSystem.StashReceiver do
  use GenServer
  require Logger
  alias PoeSystem.{Repo, LatestStash, Stash, RateLimit, RateLimitParser}
  alias PoeSystem.Items.Item
  alias PoeSystem.Items
  import Ecto.Query
  alias Ecto.Multi
  alias PoeSystem.StashReceiver.Client
  alias RustPoe.Native
  alias Req.Response

  @options NimbleOptions.new!(
             interval: [type: :pos_integer, default: :timer.seconds(1)],
             long_interval: [type: :pos_integer, default: :timer.seconds(60)],
             plug: [type: :any, default: nil],
             access_token: [required: true, type: :string],
             disabled: [type: :boolean],
             league: [type: {:list, :string}, default: []]
           )

  def start_link(_) do
    opts = Application.fetch_env!(:poe_system, __MODULE__)
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(init_arg) do
    opts =
      NimbleOptions.validate!(init_arg, @options)
      |> Enum.into(%{})

    if not Map.get(opts, :disabled, false) do
      Process.send_after(self(), :cycle, opts.interval)
    end

    {:ok, opts}
  end

  @impl true
  def handle_info(:cycle, state) do
    if ratelimit_allowed?(state) do
      ls =
        case Repo.one(from LatestStash, select: [:id]) do
          nil -> nil
          a -> a.id
        end

      resp = Client.get_stash_data(ls, state)
      process_stash(resp, ls, state)
      limits = parse_and_set_ratelimits(resp)
      {:noreply, Map.put(state, :limits, limits)}
    else
      send(self(), :ratelimited)
      {:noreply, state}
    end
  end

  def handle_info(:ratelimited, state) do
    :telemetry.execute(
      [:stash_receiver, :process_stash, :ratelimited],
      %{value: state.interval},
      %{retry_after: false, header: true}
    )

    Logger.info(message: "ratelimited by internal state", interval: state.interval)

    Process.send_after(self(), :cycle, state.interval)
    {:noreply, state}
  end

  def handle_info({:ratelimited, retry_after}, state) do
    :telemetry.execute(
      [:stash_receiver, :process_stash, :ratelimited],
      %{value: retry_after},
      %{retry_after: true, header: false}
    )

    Logger.info(message: "ratelimited by api", interval: retry_after)

    Process.send_after(self(), :cycle, :timer.seconds(retry_after))
    {:noreply, state}
  end

  defp process_stash(resp, ls, state)

  defp process_stash(%Response{status: 429} = resp, _, _) do
    retry_after =
      Map.fetch!(resp.headers, "retry-after")
      |> List.first()
      |> String.to_integer()

    send(self(), {:ratelimited, retry_after})
  end

  defp process_stash(%Response{status: status}, _, state) when status != 200 do
    send(self(), {:ratelimited, state.long_interval})
  end

  defp process_stash(resp, ls, state) do
    start = System.monotonic_time()
    insert_stash_data(resp, ls, state.league)
    delta = System.monotonic_time() - start

    :telemetry.execute(
      [:stash_receiver, :process_stash, :done],
      %{delta: delta}
    )

    Process.send_after(self(), :cycle, state.interval)
  end

  defp parse_and_set_ratelimits(resp) do
    %{
      "x-rate-limit-policy" => policy,
      "x-rate-limit-rules" => rules
    } = resp.headers

    policy = List.first(policy)

    rules =
      String.split(Enum.join(rules, ","), ",")
      |> Enum.map(&String.downcase/1)

    for rule <- rules do
      rules_header = "x-rate-limit-#{rule}"
      rules_state_header = "x-rate-limit-#{rule}-state"

      %{
        ^rules_header => limit,
        ^rules_state_header => limit_state
      } = resp.headers

      {
        :ok,
        limits,
        _,
        _,
        _,
        _
      } = RateLimitParser.limits(Enum.join(limit, ","))

      {
        :ok,
        limits_states,
        _,
        _,
        _,
        _
      } = RateLimitParser.limits(Enum.join(limit_state, ","))

      limits_states
      |> Enum.with_index()
      |> Enum.map(fn {ls, idx} -> set_ratelimit_state(policy, rule, idx, ls) end)

      limits
      |> Enum.with_index()
      |> Enum.map(fn {[req, sec, _], idx} ->
        {ratelimit_key(policy, rule, idx), req, sec}
      end)
    end
    |> Enum.flat_map(fn x -> x end)
  end

  defp ratelimit_key(policy, rule, idx) do
    "#{policy}_#{rule}_#{idx}"
  end

  defp set_ratelimit_state(policy, rule, idx, [req, sec, _penalty]) do
    RateLimit.set(ratelimit_key(policy, rule, idx), :timer.seconds(sec), req)
  end

  defp ratelimit_allowed?(state)

  defp ratelimit_allowed?(%{limits: limits}) do
    # just check limit because currect state sent by api and applied already
    for {key, req, sec} <- limits do
      count = RateLimit.get(key, :timer.seconds(sec))
      count < req
    end
    |> Enum.all?()
  end

  defp ratelimit_allowed?(%{}) do
    true
  end

  defp insert_stash_data(%Response{} = public_stash_resp, ls, allowed_leagues) do
    Logger.info(message: "received stash data", id: ls)

    {:ok, public_stash} = Native.process_stash_data(public_stash_resp.body)
    next_change_id = Map.fetch!(public_stash, "next_change_id")

    if next_change_id != "" do
      stash_data =
        for {stash_id, [stash_league, items]} <- public_stash["stashes"],
            item <- items,
            reduce: %{
              stashes: [],
              items: [],
              incoming_leagues: MapSet.new(),
              processed_leagues: MapSet.new()
            } do
          acc ->
            if length(allowed_leagues) == 0 or stash_league in allowed_leagues do
              sv = %{id: stash_id, item_id: item["id"]}

              item =
                item
                |> Items.into_elixir_items()
                # insert_all requires map with atom keys
                |> Map.from_struct()
                |> Map.delete(:__meta__)
                # does not provide nil id because insert_all wont properly delegate
                # its autogeneration to adapter/db
                |> Map.delete(:id)

              Map.update(acc, :stashes, [sv], &[sv | &1])
              |> Map.update(:items, [item], &[item | &1])
              |> Map.update(:processed_leagues, MapSet.new(), &MapSet.put(&1, stash_league))
            else
              acc
              |> Map.update(:incoming_leagues, MapSet.new(), &MapSet.put(&1, stash_league))
            end
        end

      Logger.info(
        incoming_leagues: MapSet.to_list(stash_data.incoming_leagues),
        processed_leagues: MapSet.to_list(stash_data.processed_leagues)
      )

      {:ok, _} =
        Multi.new()
        |> Multi.run(:remove_items_ids, fn repo, _changes ->
          ids =
            repo.all(
              from s in Stash, where: s.id in ^public_stash["remove_stashes"], select: s.item_id
            )

          {:ok, ids}
        end)
        |> Multi.delete_all(:remove_items, fn %{remove_items_ids: ids} ->
          from(i in Item, where: i.item_id in ^ids)
        end)
        |> Multi.delete_all(:remove_stashes, fn _ ->
          from(i in Stash, where: i.id in ^public_stash["remove_stashes"])
        end)
        |> Multi.insert_all(
          :insert_stashes,
          "stashes",
          stash_data.stashes,
          returning: false
        )
        |> Multi.insert_all(
          :insert_items,
          Item,
          stash_data.items,
          returning: false,
          on_conflict: {:replace, [:price]},
          conflict_target: :item_id
        )
        |> then(fn
          m when is_nil(ls) -> m
          m -> Multi.delete(m, :delete_latest_id, %LatestStash{id: ls})
        end)
        |> then(fn
          m ->
            Multi.insert(
              m,
              :insert_latest_id,
              LatestStash.changeset(%LatestStash{}, %{
                id: next_change_id
              })
            )
        end)
        |> Repo.transaction()
    end
  end
end
