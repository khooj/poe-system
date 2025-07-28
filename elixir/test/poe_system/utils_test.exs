defmodule PoeSystem.UtilsTest do
  use ExUnit.Case

  describe "to_string_key_map" do
    test "convert" do
      assert %{"a" => ["v", ["c", %{"d" => %{"g" => 1}}]]} = 
        Utils.to_string_key_map(%{a: ["v", ["c", %{d: %{g: 1}}]]})
    end
  end
end
