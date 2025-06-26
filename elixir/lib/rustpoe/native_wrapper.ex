defmodule RustPoe.NativeWrapper do
  alias RustPoe.Native
  alias PoeSystem.Items
  require Logger

  @spec closest_item(map(), [map()]) :: {:ok, map() | nil} | {:error, any()}
  def closest_item(req_item, items) when is_map(req_item) and is_list(items) do
    Logger.debug("NativeWrapper.closest_item: #{req_item["name"]}")

    case Native.closest_item(req_item, Items.into_native_items(items)) do
      {:ok, nil} = a ->
        a

      {:ok, result} ->
        {:ok, Items.into_elixir_items(result)}

      {:error, _} = a ->
        a
    end
  end
end
