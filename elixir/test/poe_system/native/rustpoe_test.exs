defmodule PoeSystem.RustPoe.Native.Test do
  alias PoeSystem.Testdata
  use ExUnit.Case

  test "validate and apply" do
    cfg = Testdata.extract_config()
    assert {:ok, validated} = RustPoe.Native.validate_and_apply_config(cfg, cfg)
    assert validated == cfg
  end

  test "check msg send" do
    assert {:ok} = RustPoe.Native.process_single_build(self())
    assert_receive {:get_items, "test"}
  end
end
