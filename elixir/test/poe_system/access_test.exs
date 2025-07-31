defmodule PoeSystem.AccessTest do
  use ExUnit.Case, async: true
  alias Utils

  test "update nested data" do
    nested = %{a: %{b: %{c: 123}}}

    assert %{a: %{b: %{c: "123"}}} =
             Iteraptor.map(nested, fn {[:a, :b, :c] = k, v} -> {k, Integer.to_string(v)} end)
  end

  describe "all_kv" do
    test "update_in" do
      nested = %{
        "a" => %{
          "stashid" => ["testleague", [1, 2, 3]],
          "stashid2" => ["testleague", [3, 4, 5]]
        }
      }

      result = %{
        "a" => %{
          "stashid" => ["testleague", ["1", "2", "3"]],
          "stashid2" => ["testleague", ["3", "4", "5"]]
        }
      }

      assert ^result =
               update_in(
                 nested,
                 [
                   "a",
                   Utils.all_kv(),
                   Access.all(),
                   Access.elem(1),
                   Access.at!(1),
                   Access.all()
                 ],
                 fn i -> Integer.to_string(i) end
               )
    end

    test "get_in" do
      nested = %{
        "a" => %{
          "stashid" => ["testleague", [1, 2, 3]],
          "stashid2" => ["testleague", [3, 4, 5]]
        }
      }

      assert [[1, 2, 3], [3, 4, 5]] =
               get_in(
                 nested,
                 [
                   "a",
                   Utils.all_kv(),
                   Access.all(),
                   Access.elem(1),
                   Access.at!(1),
                   Access.all()
                 ]
               )
    end
  end
end
