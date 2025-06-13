defmodule RustPoe.NativeWrapper do
  alias RustPoe.Native
  require Logger

  @spec closest_item(map(), [map()]) :: {:ok, map() | nil} | {:error, any()}
  def closest_item(req_item, items) do
    Logger.debug("NativeWrapper.closest_item: #{req_item["name"]}")

    res =
      items
      |> Enum.map(&Utils.to_string_key_map/1)

    case Native.closest_item(req_item, res) do
      {:ok, nil} = a ->
        a

      {:ok, result} ->
        {:ok, Utils.unsafe_to_atom_key_map(result)}

      {:error, _} = a ->
        a
    end
  end
end
