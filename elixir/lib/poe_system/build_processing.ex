defmodule PoeSystem.BuildProcessing do
  require Logger
  alias PoeSystem.Repo
  alias PoeSystem.BuildInfo
  alias PoeSystem.BuildInfo.BuildData
  alias PoeSystem.Items.Item
  alias PoeSystem.BuildInfo.RequiredItem
  alias PoeSystem.Items
  alias RustPoe.Native
  use GenServer
  import Ecto.Query

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  def start_processing_build(id) do
    GenServer.cast(__MODULE__, {:process, id})
  end

  @impl true
  def init(init_arg) do
    {:ok, init_arg}
  end

  @impl true
  def handle_cast({:process, id}, state) do
    build = BuildInfo.get_build(id)
    build_data = process_single_build(build.data)

    build_attrs =
      %{}
      |> Map.put(:processed, true)
      |> Map.put(:data, build_data)

    {:ok, _} = BuildInfo.update_build(build, build_attrs)
    Logger.debug("end processing")

    {:noreply, state}
  end

  # @spec process_single_build(BuildData) :: BuildData
  def process_single_build(build) do
    {:ok, boots} = find_similar(get_in(build["provided"]["boots"]["item"]))

    build = put_in(build["found"]["boots"], boots)

    build
  end

  # @spec find_similar(%RequiredItem{}) :: {:ok, [%Item{}] | nil}
  def find_similar(item) do
    {:ok, mods} = Native.extract_mods_for_search(item)

    items_stream =
      Items.search_items_by_attrs_query(
        basetype: opt(item["search_basetype"], item["basetype"]),
        category: opt(item["search_category"], item["category"]),
        subcategory: opt(item["search_subcategory"], item["subcategory"]),
        mods: mods
      )
      # |> limit(500)
      |> Repo.stream()

    {:ok, items} =
      Repo.transaction(fn ->
        Enum.to_list(items_stream)
      end)

    Native.closest_item(item, items)
  end

  defp opt(false, _), do: nil
  defp opt(nil, _), do: nil
  defp opt(_, v), do: v
end
