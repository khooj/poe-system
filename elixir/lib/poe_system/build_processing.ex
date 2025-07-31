defmodule PoeSystem.BuildProcessing do
  require Logger
  alias Phoenix.PubSub
  alias RustPoe.Native
  alias PoeSystem.Repo
  alias PoeSystem.Build
  alias PoeSystem.Build.{FoundItems, ProvidedItems}
  alias PoeSystem.Items
  alias PoeSystem.Items.{Item, NativeItem}
  alias PoeSystem.PoeNinja
  alias RustPoe.Native
  alias PoeSystem.BuildProcessing.Mods
  use Oban.Worker, queue: :new_builds
  use Telemetria
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
  @telemetria level: :info, group: :poe1_build_processing
  def perform(%Oban.Job{args: %{"id" => id} = _args}) do
    build = Build.get_build(id)
    build_found = process_single_build(build.provided)

    {:ok, _} =
      Build.changeset(build, %{
        processed: true,
        found: build_found
      })
      |> Repo.update()

    PubSub.broadcast!(PoeSystem.PubSub, "build:#{id}", {PoeSystem.PubSub, "done"})
    Logger.debug("end processing")

    :ok
  end

  @impl Oban.Worker
  def backoff(_job) do
    5
  end

  @spec process_single_build(ProvidedItems.t()) :: FoundItems.t()
  # FIXME: find out how to exclude func args from span (cannot encode in json)
  # @telemetria level: :info, group: :poe1_build_processing, locals: []
  def process_single_build(provided) do
    found =
      provided
      |> Map.from_struct()
      |> Enum.map(fn {k, v} -> {k, process_entry(v)} end)
      |> Enum.into(%{})

    struct!(FoundItems, found)
  end

  @spec process_entry([NativeItem.t()] | NativeItem.t() | nil) :: Item.t() | nil
  # @telemetria level: :info, group: :poe1_build_processing, locals: []
  defp process_entry(data)

  defp process_entry(nil), do: nil

  defp process_entry(items) when is_list(items) do
    result =
      items
      |> Enum.map(fn a ->
        find_similar(a, Native.get_stored_item_type(a.item))
      end)
      |> Enum.reject(&is_nil/1)

    Logger.debug("found items for few items")
    result
  end

  defp process_entry(item) do
    result = find_similar(item, Native.get_stored_item_type(item.item))
    Logger.debug("found item for single item: #{result && result.id}")
    result
  end

  def find_similar(%NativeItem{item: %Item{rarity: "unique"} = item}, {:ok, _}) do
    case PoeNinja.get_item(item.name) do
      {:ok, nil} ->
        nil

      {:ok, val} ->
        %{item | price: {:chaos, val.chaos}}
    end
  end

  @spec find_similar(NativeItem.t(), {:ok, atom()}) :: Item.t() | nil
  # @telemetria level: :info, group: :poe1_build_processing, locals: []
  def find_similar(item, {:ok, _t}) do
    Logger.debug("extract mods")
    items_stream = Mods.extract_options_for_search(item)
    process_items_stream(items_stream, item)
  end

  @spec process_items_stream(
          Ecto.Query.t(),
          NativeItem.t(),
          Ecto.UUID.t() | nil,
          Item.t() | nil
        ) ::
          Item.t() | nil
  # @telemetria level: :info, group: :poe1_build_processing, locals: []
  defp process_items_stream(query, req_item, last_id \\ nil, last_item \\ nil)

  defp process_items_stream(query, req_item, last_id, last_item) do
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

  @spec closest_item(NativeItem.t(), [Item.t()], Item.t() | nil) ::
          {:ok, Item.t() | nil} | Native.nif_err()
  defp closest_item(req_item, items, last_item)

  defp closest_item(req_item, items, nil), do: Native.closest_item(req_item, items)

  defp closest_item(req_item, items, last_item),
    do: Native.closest_item(req_item, [last_item | items])

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
