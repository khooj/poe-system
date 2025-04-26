defmodule TestStruct do
  defstruct([:a, :b])
end

defmodule PoeSystem.RustPoe.Native.Test do
  require Logger
  alias PoeSystem.Testdata
  use ExUnit.Case

  test "validate and apply" do
    cfg = Testdata.extract_config()
    assert {:ok, validated} = RustPoe.Native.validate_and_apply_config(cfg, cfg)
    assert validated == cfg
  end
end
