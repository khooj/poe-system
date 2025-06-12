defmodule PoeSystem.StashReceiver do
  use GenServer
  alias PoeSystem.{Repo, LatestStash, Stash, RateLimit, RateLimitParser}
  alias PoeSystem.Items.Item
  import Ecto.Query
  alias Ecto.Multi
  alias PoeSystem.StashReceiver.Client
  alias RustPoe.Native
  alias Req.Response

  @options NimbleOptions.new!(
             interval: [required: true, type: :pos_integer],
             plug: [type: :any, default: nil],
             access_token: [required: true, type: :string],
             test: [type: :boolean]
           )

  def start_link(_) do
    opts = Application.fetch_env!(:poe_system, __MODULE__)
    opts = NimbleOptions.validate!(opts, @options)
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @impl true
  def init(init_arg) do
    opts = Enum.into(init_arg, %{})

    if not Map.get(opts, :test, false) do
      Process.send_after(self(), :cycle, opts.interval)
    end

    {:ok, opts}
  end

  @impl true
  def handle_info(:cycle, state) do
    ls = Repo.one(from LatestStash, select: [:id])

    Client.get_stash_data(ls, state)
    |> receive_stash(ls, state)

    {:noreply, state}
  end

  defp receive_stash(resp, ls, state)

  defp receive_stash(%Response{status: 429} = resp, _, _) do
    retry_after =
      Map.fetch!(resp.headers, "retry-after")
      |> List.first()
      |> String.to_integer()

    Process.send_after(self(), :cycle, :timer.seconds(retry_after + 1))
  end

  defp receive_stash(resp, ls, state) do
    %{
      "x-rate-limit-policy" => policy,
      "x-rate-limit-rules" => rules
    } = resp.headers

    rules =
      String.split(rules, ",")
      |> Enum.map(&String.downcase/2)

    limits =
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
        } = RateLimitParser.limits(limit)

        {
          :ok,
          limits_states,
          _,
          _,
          _,
          _
        } = RateLimitParser.limits(limit_state)

        limits_states
        |> Enum.with_index()
        |> Enum.map(fn {ls, idx} -> set_ratelimit_state(policy, rule, idx, ls) end)

        {rule, limits}
      end

    if ratelimit_allowed?(policy, limits) do
      process_stash(resp, ls)
    end

    Process.send_after(self(), :cycle, state.interval)
  end

  defp ratelimit_key(policy, rule, idx) do
    "#{policy}_#{rule}_#{idx}"
  end

  defp set_ratelimit_state(policy, rule, idx, [req, sec, _penalty]) do
    RateLimit.set(ratelimit_key(policy, rule, idx), :timer.seconds(sec), req)
  end

  defp ratelimit_allowed?(policy, limits) do
    allowed? =
      for {rule, limits} <- limits do
        limits
        |> Enum.with_index()
        |> Enum.all?(fn {[req, sec, _], idx} ->
          count = RateLimit.get(ratelimit_key(policy, rule, idx), :timer.seconds(sec))
          count < req
        end)
      end
      |> Enum.all?()

    if allowed? do
      for {rule, limits} <- limits do
        limits
        |> Enum.with_index()
        |> Enum.map(fn {[req, sec, _], idx} ->
          {:allow, _} = RateLimit.hit(ratelimit_key(policy, rule, idx), :timer.seconds(sec), req)
        end)
      end
    end

    allowed?
  end

  defp process_stash(%Response{} = public_stash_resp, ls) do
    {:ok, public_stash} = Native.process_stash_data(public_stash_resp.body)

    stash_data =
      for {stash_id, items} <- public_stash["stashes"],
          item <- items,
          reduce: %{} do
        acc ->
          sv = %{id: stash_id, item_id: item["id"]}

          Map.update(acc, :stashes, [sv], &[sv | &1])
          |> Map.update(:items, [item], &[item | &1])
      end

    {:ok, _} =
      Multi.new()
      |> Multi.run(:remove_items_ids, fn repo, _changes ->
        ids =
          repo.all(
            from s in Stash, where: s.id in ^public_stash["remove_stashes"], select: [s.item_id]
          )

        {:ok, ids}
      end)
      |> Multi.delete_all(:remove_items, fn %{remove_items_ids: ids} ->
        from(i in Item, where: i.id in ^ids)
      end)
      |> Multi.delete_all(:remove_stashes, fn _ ->
        from(i in Stash, where: i.id in ^public_stash["remove_stashes"])
      end)
      |> then(
        &Enum.reduce(Enum.with_index(stash_data.stashes), &1, fn
          # some entries can be with empty stash id
          # probably private stashes somehow made it into response
          {%{id: ""}, _}, acc ->
            acc

          {el, idx}, acc ->
            Multi.insert(acc, {:insert_stash_id, idx}, Stash.changeset(%Stash{}, el))
        end)
      )
      |> then(
        &Enum.reduce(Enum.with_index(stash_data.items), &1, fn {el, idx}, acc ->
          Multi.insert(&1, {:insert_item, idx}, Item.changeset(%Item{}, el))
        end)
      )
      |> then(fn
        m when is_nil(ls) -> m
        m -> Multi.delete(m, :delete_latest_id, %LatestStash{id: ls})
      end)
      |> Multi.insert(:insert_latest_id, %LatestStash{
        id: Map.fetch!(public_stash, "next_change_id")
      })
      |> Repo.transaction()
  end
end
