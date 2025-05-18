defmodule RustPoe.Native do
  use Rustler,
    otp_app: :poe_system,
    crate: :elixir,
    path: "../rust/elixir"

  def extract_build_config(_pobxml, _itemset, _skillset), do: error()
  def validate_and_apply_config(_extracted_config, _user_config), do: error()
  def validate_config(_config), do: error()
  def process_single_build(_pid), do: error()
  def extract_mods_for_search(_req_item), do: error()
  def closest_item(_item, _items), do: error()
  def get_items_from_stash_data(_data), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
