defmodule PoeSystem.BuildProcessing do
  require Logger
  alias Phoenix.PubSub
  alias RustPoe.NativeWrapper
  alias PoeSystem.Repo
  alias PoeSystem.Build
  alias PoeSystem.Build.{BuildInfo, RequiredItem}
  alias PoeSystem.Items
  alias PoeSystem.Items.Item
  alias RustPoe.Native
  use Oban.Worker, queue: :new_builds
  import Ecto.Query

  @items_per_tx Application.compile_env!(:poe_system, [PoeSystem.BuildProcessing, :items_per_tx])

  def queue_processing_build_multi(multi, name, id_fn) do
    multi
    |> Oban.insert(name, &id_fn.(&1))
  end

  @spec queue_processing_build(String.t()) :: {:ok, Oban.Job.t()} | {:error, Ecto.Changeset.t()}
  def queue_processing_build(id) do
    Oban.insert(new(%{id: id}))
  end

  @impl Oban.Worker
  def perform(%Oban.Job{args: %{"id" => id} = _args}) do
    build = Build.get_build(id)
    build_data = process_single_build(build.data)

    build_attrs =
      %{}
      |> Map.put(:processed, true)
      |> Map.put(:data, build_data)

    {:ok, _} = Build.update_build(build, build_attrs)
    PubSub.broadcast!(PoeSystem.PubSub, "build:#{id}", {PoeSystem.PubSub, "done"})
    Logger.debug("end processing")

    :ok
  end

  @impl Oban.Worker
  def backoff(_job) do
    5
  end

  @spec process_single_build(BuildInfo.t()) :: BuildInfo.t()
  def process_single_build(build) do
    found =
      build["provided"]
      |> Enum.map(fn {k, v} -> {k, process_entry(v)} end)
      |> Enum.into(%{})

    put_in(build["found"], found)
  end

  @spec process_entry(nil) :: nil
  defp process_entry(nil), do: nil

  @spec process_entry([%{String.t() => RequiredItem.t()}]) :: [Item.t()] | []
  defp process_entry(items) when is_list(items) do
    result =
      items
      |> Enum.map(fn a ->
        find_similar(a["item"], Native.get_req_item_type(a["item"]["info"]))
      end)
      |> Enum.reject(&is_nil/1)

    Logger.debug("found items for few items: #{List.first(result)["id"]}")
    result
  end

  @spec process_entry(%{String.t() => RequiredItem.t()}) :: Item.t() | nil
  defp process_entry(item) do
    result = find_similar(item["item"], Native.get_req_item_type(item["item"]["info"]))
    Logger.debug("found item for single item: #{result["id"]}")
    result
  end

  @spec find_similar(RequiredItem.t(), {:ok, :gem}) :: Item.t() | nil
  def find_similar(item, {:ok, :gem}) do
    name = item["basetype"]
    {:ok, quality, level} = Native.extract_gem_props(item)

    items_stream =
      Items.search_gems_by_attrs_query(name, quality, level)

    process_items_stream(items_stream, item)
  end

  @spec find_similar(RequiredItem.t(), {:ok, :flask}) :: Item.t() | nil
  def find_similar(item, {:ok, :flask}) do
    {:ok, mods} = Native.extract_mods_for_search(item)
    {:ok, quality} = Native.extract_flask_props(item)

    items_stream =
      Items.search_items_by_attrs_query(
        mods,
        basetype: item["basetype"]
      )
      |> Items.append_flask_quality(quality)

    process_items_stream(items_stream, item)
  end

  @spec find_similar(RequiredItem.t(), {:ok, atom()}) :: Item.t() | nil
  def find_similar(item, {:ok, _}) do
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

  @spec process_items_stream(
          Ecto.Query.t(),
          RequiredItem.t(),
          Ecto.UUID.t() | nil,
          Item.t() | nil
        ) ::
          Item.t() | nil
  defp process_items_stream(query, req_item, last_id \\ nil, last_item \\ nil) do
    {:ok, items} =
      Repo.transaction(fn ->
        query
        |> limit(@items_per_tx)
        |> Items.append_id_cursor(last_id)
        |> Repo.stream()
        |> Enum.to_list()
      end)

    if Enum.empty?(items) do
      last_item
    else
      new_last_id = List.last(items).id

      {:ok, result} = closest_item(req_item, items, last_item)

      process_items_stream(query, req_item, new_last_id, result)
    end
  end

  @spec closest_item(RequiredItem.t(), [Item.t()], nil) ::
          {:ok, Item.t() | nil} | Native.nif_err()
  defp closest_item(req_item, items, nil), do: NativeWrapper.closest_item(req_item, items)

  @spec closest_item(RequiredItem.t(), [Item.t()], Item.t()) ::
          {:ok, Item.t() | nil} | Native.nif_err()
  defp closest_item(req_item, items, last_item),
    do: NativeWrapper.closest_item(req_item, [last_item | items])

  defp opt(false, _), do: nil
  defp opt(nil, _), do: nil
  defp opt(_, v), do: v

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
