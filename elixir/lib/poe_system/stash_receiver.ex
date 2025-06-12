defmodule PoeSystem.StashReceiver do
  use GenServer
  alias PoeSystem.{Repo, LatestStash, Stash}
  alias PoeSystem.Items.Item
  import Ecto.Query
  alias Ecto.Multi
  alias PoeSystem.StashReceiver.Client
  alias RustPoe.Native

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
    if not Keyword.get(init_arg, :test) do
      Process.send_after(self(), :cycle, Keyword.fetch!(init_arg, :interval))
    end

    {:ok, init_arg}
  end

  @impl true
  def handle_info(:cycle, state) do
    ls = Repo.one(from LatestStash, select: [:id])

    Client.get_next_stash(ls, state)
    |> receive_stash(ls)
    |> Repo.transaction()

    Process.send_after(self(), :cycle, Keyword.fetch!(state, :interval))

    {:noreply, state}
  end

  def receive_stash(public_stash_resp, ls) do
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
      &Enum.reduce(Enum.with_index(stash_data.stashes), &1, fn {el, idx}, acc ->
        Multi.insert(acc, {:insert_stash_id, idx}, Stash.changeset(%Stash{}, el))
      end)
    )
    |> then(
      &Enum.reduce(Enum.with_index(stash_data.items), &1, fn {el, idx}, acc ->
        Multi.insert(&1, {:insert_item, idx}, Item.changeset(%Item{}, el))
      end)
    )
    |> Multi.delete(:delete_latest_id, %LatestStash{id: ls})
    |> Multi.insert(:insert_latest_id, %LatestStash{id: public_stash["next_change_id"]})
  end
end
