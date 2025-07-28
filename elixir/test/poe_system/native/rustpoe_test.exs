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

  setup do
    :rarity
    :ok
  end

  test "extract build config" do
    Testdata.extract_config()
  end

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
               RustPoe.Native.closest_item(cfg.provided.helmet, items)

      assert result != nil
    end

    test "raise w/ nil" do
      cfg = Testdata.extract_config()
      items = Repo.all(Item)

      assert_raise(ArgumentError, fn ->
        RustPoe.Native.closest_item(cfg.provided.body, [nil | items])
      end)
    end

    test "returns nil" do
      cfg = Testdata.extract_config()
      item = Repo.one(from Item, limit: 1)

      assert {:ok, nil} =
               RustPoe.Native.closest_item(cfg.provided.body, [item])
    end
  end

  test "process_stash_data" do
    stash = Testdata.stash_json()
    assert {:ok, data} = RustPoe.Native.process_stash_data(stash)
  end

  test "ecto ch" do
    [item | _] = Testdata.items()

    %Item{}
      |> Item.internal_change(Map.from_struct(item))
      |> Repo.insert!()
  end
end
