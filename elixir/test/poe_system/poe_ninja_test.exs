defmodule PoeSystem.PoeNinjaTest do
  use ExUnit.Case
  use PoeSystemWeb.ConnCase
  alias PoeSystem.{PoeNinja}

  test "process items from api" do
    Req.Test.stub(Client, fn conn ->
      conn
      |> Req.Test.json(%{
        "lines" => [
          %{"name" => "Original Sin", "chaosValue" => 100.0, "divineValue" => 1.0}
        ]
      })
    end)

    assert {:noreply, _} =
             PoeNinja.handle_info({:refresh, "UniqueWeapon"}, plug: {Req.Test, Client})

    assert {:reply, {:ok, v}, _} = PoeNinja.handle_call({:item, "Original Sin"}, self(), [])
    assert %{chaos: 100.0, divine: 1.0} = v
  end

  test "refresh_all" do
    Req.Test.stub(Client, fn conn ->
      conn
      |> Req.Test.json(%{
        "lines" => [
          %{"name" => "Original Sin", "chaosValue" => 100.0, "divineValue" => 1.0}
        ]
      })
    end)

    assert {:noreply, _} =
             PoeNinja.handle_info(:refresh_all, interval: 10, jitter_start: 0, jitter_end: 10)

    assert_receive {:refresh, _}
    assert_receive :refresh_all
  end
end
