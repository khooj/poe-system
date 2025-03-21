defmodule RustPoe.Native do
  use Rustler,
    otp_app: :poe_system,
    crate: :elixir,
    path: "../rust/elixir"

  def extract_build_config(_pobxml, _itemset, _skillset), do: error()
  def validate_and_apply_config(_extracted_config, _user_config), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
