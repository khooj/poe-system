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
      |> Map.put(:info, build_data)

    {:ok, _} = BuildInfo.update_build(build, build_attrs)
    Logger.debug("end processing")

    :ok
  end

  def process_single_build(build) do
    found =
      build["provided"]
      |> Enum.map(fn {k, v} -> {k, process_entry(v)} end)
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
    Native.closest_item(item, items)
  end

  defp opt(false, _), do: nil
  defp opt(nil, _), do: nil
  defp opt(_, v), do: v

  defp keys_to_string(%{__struct__: _} = v) do
    keys_to_string(Map.from_struct(v))
  end

  defp keys_to_string(v) do
    v
    |> Enum.map(fn {k, v} -> {Atom.to_string(k), v} end)
    |> Enum.into(%{})
  end

  def test do
    {:ok, _} = queue_processing_build("cb5bb0be-405a-4328-accc-6e8dadbe6397")
  end

  def drop_jobs do
    a = Atom.to_string(__MODULE__)

    Oban.Job
    |> Ecto.Query.where(worker: ^a)
    |> Oban.delete_all_jobs()
  end
end
