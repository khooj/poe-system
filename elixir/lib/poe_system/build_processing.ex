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

  @impl Oban.Worker
  def backoff(_job) do
    5
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
    result =
      items
      |> Enum.map(fn a -> find_similar(a["item"]) end)
      |> Enum.filter(&(not is_nil(&1)))

    Logger.debug("found items for few items: #{List.first(result)["id"]}")
    result
  end

  defp process_entry(item) do
    result = find_similar(item["item"])
    Logger.debug("found item for single item: #{result["id"]}")
    result
  end

  def find_similar(item) do
    Logger.debug("extract mods")
    {:ok, mods} = Native.extract_mods_for_search(item)

    if Enum.empty?(mods) do
      nil
    else
      Logger.debug("search_items")

      items_stream =
        Items.search_items_by_attrs_query(
          mods,
          basetype: opt(item["search_basetype"], item["basetype"]),
          category: opt(item["search_category"], item["category"]),
          subcategory: opt(item["search_subcategory"], item["subcategory"])
        )

      process_items_stream(items_stream, item)
    end
  end

  defp process_items_stream(query, req_item, last_id \\ nil, last_item \\ nil) do
    {:ok, items} =
      Repo.transaction(fn ->
        query
        |> limit(500)
        |> Items.append_id_cursor(last_id)
        |> Repo.stream()
        |> Stream.map(&keys_to_string/1)
        |> Enum.to_list()
      end)

    if Enum.empty?(items) do
      last_item
    else
      new_last_id = List.last(items)["id"]

      result =
        if last_item do
          {:ok, i} = Native.closest_item(req_item, [last_item | items])
          i
        else
          {:ok, i} = Native.closest_item(req_item, items)
          i
        end

      process_items_stream(query, req_item, new_last_id, result)
    end
  end

  defp opt(false, _), do: nil
  defp opt(nil, _), do: nil
  defp opt(_, v), do: v

  defp keys_to_string(%{__struct__: _} = v) do
    keys_to_string(Map.from_struct(v))
  end

  defp keys_to_string(nil) do
    Logger.critical("nil keys_to_string")
    nil
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
