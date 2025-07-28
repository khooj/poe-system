defmodule RustPoe.NativeWrapperExc do
  alias RustPoe.Native
  alias PoeSystem.Items
  alias PoeSystem.Items.NativeItem
  alias PoeSystem.Build.{ProvidedItems, FoundItems}
  require Logger
  alias Utils

  @spec closest_item(NativeItem.t(), [map()]) :: {:ok, map() | nil} | {:error, any()}
  def closest_item(req_item, items) when is_map(req_item) and is_list(items) do
    req_json = NativeItem.into_json(req_item)

    case Native.closest_item(req_json, Items.into_native_items(items)) do
      {:ok, nil} = a ->
        a

      {:ok, result} ->
        {:ok, Items.into_elixir_items(result)}

      {:error, _} = a ->
        a
    end
  end

  def extract_build_config(pobdata, itemset, skillset, profile \\ "simpleeverything") do
    Native.extract_build_config(pobdata, itemset, skillset, profile)
    #
    # provided = build["provided"]
    #   |> ProvidedItems.from_json()
    #
    # found = build["found"]
    #   |> FoundItems.from_json()
    #
    # {:ok, %{provided: provided, found: found}}
  end

  def process_stash_data(body) do
    {:ok, public_stash} = Native.process_stash_data(body)

    res = public_stash
      |> update_in([
        "stashes", 
        Utils.all_kv(), 
        Access.elem(1),
        Access.at!(1),
        Access.all(),
        "info"
      ], fn it -> ItemInfo.from_json(it) end)

    {:ok, res}
  end
end
