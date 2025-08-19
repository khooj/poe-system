defmodule PoeSystem.RemoveExcessiveBasetypesTest do
  alias PoeSystem.Testdata
  alias PoeSystem.RemoveExcessiveBasetypes
  use PoeSystem.DataCase
  use ExUnit.Case, async: true

  describe "basic work" do
    setup do
      Testdata.insert_items()
      :ok
    end

    test "basic" do
      assert RemoveExcessiveBasetypes.remove_items()
    end
  end
end
