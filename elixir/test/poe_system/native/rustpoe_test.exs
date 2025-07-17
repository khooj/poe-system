defmodule TestStruct do
  defstruct([:a, :b])
end

defmodule PoeSystem.RustPoe.Native.Test do
  require Logger
  require Ecto.Query
  alias PoeSystem.Items.Item
  alias PoeSystem.Repo
  alias PoeSystem.Testdata
  use PoeSystem.DataCase
  use ExUnit.Case, async: true

  test "fill configs by rule" do
    cfg = Testdata.extract_config()
    assert {:ok, cfg_filled} = RustPoe.Native.fill_configs_by_rule(cfg, "simplenores")
    assert cfg_filled != cfg
  end

  describe "closest item" do
    setup do
      Testdata.insert_items()
      :ok
    end

    test "success" do
      cfg = Testdata.extract_config(early_setup: true)
      items = Repo.all(Item)

      assert {:ok, result} =
               RustPoe.NativeWrapper.closest_item(cfg["provided"]["helmet"]["item"], items)

      assert result != nil
    end

    test "raise w/ nil" do
      cfg = Testdata.extract_config()
      items = Repo.all(Item)

      assert_raise(FunctionClauseError, fn ->
        RustPoe.NativeWrapper.closest_item(cfg["provided"]["body"]["item"], [nil | items])
      end)
    end

    test "returns nil" do
      cfg = Testdata.extract_config()
      items = Repo.all(Item)

      assert {:ok, nil} =
               RustPoe.NativeWrapper.closest_item(cfg["provided"]["body"]["item"], [
                 List.first(items)
               ])
    end
  end
end
