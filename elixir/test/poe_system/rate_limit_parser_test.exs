defmodule PoeSystem.RateLimitParserTest do
  use PoeSystemWeb.ConnCase
  alias PoeSystem.RateLimitParser
  use ExUnit.Case, async: true

  test "parse simple limit" do
    assert {:ok, [[1, 2, 3]], _, _, _, _} = RateLimitParser.limits("1:2:3")
  end

  test "parse multiple limits" do
    res = RateLimitParser.limits("1:2:3,10:20:100,11:22:33")

    assert {
             :ok,
             [[1, 2, 3], [10, 20, 100], [11, 22, 33]],
             _,
             _,
             _,
             _
           } = res
  end
end
