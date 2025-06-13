defmodule TestStruct do
  defstruct([:a, :b])
end

defmodule PoeSystem.RustPoe.Native.Test do
  require Logger
  require Ecto.Query
  alias PoeSystem.Items
  alias PoeSystem.Items.Item
  alias PoeSystem.Repo
  alias PoeSystem.Testdata
  use PoeSystem.DataCase

  test "validate and apply" do
    cfg = Testdata.extract_config()
    assert {:ok, validated} = RustPoe.Native.validate_and_apply_config(cfg, cfg)
    assert validated == cfg
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
