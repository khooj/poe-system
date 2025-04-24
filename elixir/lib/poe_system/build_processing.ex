defmodule PoeSystem.BuildProcessing do
  require Logger
  alias PoeSystem.Repo
  alias PoeSystem.BuildInfo
  alias PoeSystem.Items
  alias RustPoe.Native
  use Oban.Worker, queue: :new_builds
  import Ecto.Query

  def queue_processing_build_multi(multi, name, id) do
    multi
    |> Oban.insert(name, new(%{id: id}))
  end

  def queue_processing_build(id) do
    Oban.insert(new(%{id: id}))
  end

  @impl Oban.Worker
  def perform(%Oban.Job{args: %{"id" => id} = _args}) do
    build = BuildInfo.get_build(id)
    build_data = process_single_build(build.data)

    build_attrs =
      %{}
      |> Map.put(:processed, true)
      |> Map.put(:data, build_data)

    {:ok, _} = BuildInfo.update_build(build, build_attrs)
    Logger.debug("end processing")

    :ok
  end

  # @spec process_single_build(BuildData) :: BuildData
  def process_single_build(build) do
    # boots = find_similar(get_in(build["provided"]["boots"]["item"]))
    # helmet = find_similar(get_in(build["provided"]["helmet"]["item"]))
    # body = find_similar(get_in(build["provided"]["body"]["item"]))
    # gloves = find_similar(get_in(build["provided"]["gloves"]["item"]))
    # weapon1 = find_similar(get_in(build["provided"]["weapon1"]["item"]))
    # weapon2 = find_similar(get_in(build["provided"]["weapon2"]["item"]))
    # ring1 = find_similar(get_in(build["provided"]["ring1"]["item"]))
    # ring2 = find_similar(get_in(build["provided"]["ring2"]["item"]))
    # belt = find_similar(get_in(build["provided"]["belt"]["item"]))
    # amulet = find_similar(get_in(build["provided"]["amulet"]["item"]))
    #
    # items = [
    #   "boots",
    #   "helmet",
    #   "body",
    #   "gloves",
    #   "weapon1",
    #   "weapon2",
    #   "ring1",
    #   "ring2",
    #   "belt",
    #   "amulet",
    #   "flasks",
    #   "gems",
    #   "jewels"
    # ]

    found =
      build["provided"]
      |> Enum.map(fn {k, v} -> {k, process_entry(IO.inspect(v))} end)
      |> Enum.into(%{})

    put_in(build["found"], found)
  end

  defp process_entry(nil), do: nil

  defp process_entry(items) when is_list(items) do
    items
    |> Enum.map(fn a -> find_similar(a["item"]) end)
    |> Enum.filter(&(not is_nil(&1)))
  end

  defp process_entry(item) do
    find_similar(item["item"])
  end

  # @spec find_similar(%RequiredItem{}) :: {:ok, [%Item{}] | nil}
  def find_similar(item) do
    Logger.debug("extract mods")
    {:ok, mods} = Native.extract_mods_for_search(item)

    Logger.debug("search_items")

    items_stream =
      Items.search_items_by_attrs_query(
        basetype: opt(item["search_basetype"], item["basetype"]),
        category: opt(item["search_category"], item["category"]),
        subcategory: opt(item["search_subcategory"], item["subcategory"]),
        mods: mods
      )
      |> limit(10)
      |> Repo.stream()

    {:ok, items} =
      Repo.transaction(fn ->
        Enum.to_list(items_stream)
        |> Enum.map(&keys_to_string/1)
      end)

    Logger.debug("closest_item")
    Native.closest_item(IO.inspect(item), IO.inspect(items))
  end

  defp opt(false, _), do: nil
  defp opt(nil, _), do: nil
  defp opt(_, v), do: v

  defp keys_to_string(v) do
    v
    |> Enum.map(fn {k, v} -> {Atom.to_string(k), v} end)
    |> Enum.into(%{})
  end
end
