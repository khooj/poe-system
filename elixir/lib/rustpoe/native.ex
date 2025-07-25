defmodule RustPoe.Native do
  use Rustler,
      [
        otp_app: :poe_system,
        crate: :elixir,
        path: "../rust/elixir"
      ] ++ Application.compile_env(:poe_system, Rustler, [])

  alias PoeSystem.Build.{BuildInfo, RequiredItem, Mod}
  alias PoeSystem.Items.Item

  @type build_preview :: %{
          itemset: String.t(),
          skillset: String.t(),
          pobData: String.t(),
          data: BuildInfo.t()
        }
  @type nif_err :: {:error, any()}
  @type item_info :: map()

  @spec extract_build_config(String.t(), String.t(), String.t()) ::
          {:ok, build_preview()} | nif_err()
  def extract_build_config(_pobxml, _itemset, _skillset), do: error()

  @spec validate_and_apply_config(BuildInfo.t(), BuildInfo.t()) ::
          {:ok, BuildInfo.t()} | nif_err()
  def validate_and_apply_config(_extracted_config, _user_config), do: error()

  @spec validate_config(BuildInfo.t()) :: :ok | nif_err()
  def validate_config(_config), do: error()

  @spec extract_mods_for_search(RequiredItem.t()) :: {:ok, [Mod.t()]} | nif_err()
  def extract_mods_for_search(_req_item), do: error()

  @spec closest_item(RequiredItem.t(), [Item.t()]) :: {:ok, Item.t() | nil} | nif_err()
  def closest_item(_item, _items), do: error()

  @spec get_items_from_stash_data(String.t()) :: {:ok, [Item.t()]} | nif_err()
  def get_items_from_stash_data(_data), do: error()

  @spec get_req_item_type(item_info()) :: {:ok, RequiredItem.item_type()} | nif_err()
  def get_req_item_type(_data), do: error()

  @spec get_stored_item_type(item_info()) :: {:ok, Item.item_type()} | nif_err()
  def get_stored_item_type(_data), do: error()

  @spec extract_gem_props(RequiredItem.t()) :: {:ok, Item.quality(), Item.level()} | nif_err()
  def extract_gem_props(_data), do: error()

  @spec extract_flask_props(RequiredItem.t()) :: {:ok, Item.quality()} | nif_err()
  def extract_flask_props(_data), do: error()

  @spec process_stash_data(String.t(), boolean()) :: {:ok, map()} | nif_err()
  def process_stash_data(_data, _without_zero_price \\ true), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
