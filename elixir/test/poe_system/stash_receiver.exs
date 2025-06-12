defmodule PoeSystem.StashReceiverTest do
  use ExUnit.Case
  use PoeSystemWeb.ConnCase
  alias PoeSystem.StashReceiver.Client
  alias PoeSystem.StashReceiver
  alias PoeSystem.Testdata
  alias Ecto.Multi
  alias PoeSystem.Repo

  setup do
    {:ok, opts} = StashReceiver.init(Application.fetch_env!(:poe_system, PoeSystem.StashReceiver))
    %{opts: opts}
  end

  test "process stash data from api", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> send_resp(200, Testdata.stash_json())
    end)

    assert {:error, :success} =
             Repo.transaction(fn ->
               StashReceiver.handle_info(:cycle, opts)

               Repo.rollback(:success)
             end)

    assert_receive :cycle
  end

  test "rate limited", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_resp_header("retry-after", "1")
      |> send_resp(429, "Timeout")
    end)

    {:noreply, _} = StashReceiver.handle_info(:cycle, opts)

    assert_receive :cycle, 1200
  end

  test "rate limited", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_resp_header("retry-after", "1")
      |> send_resp(429, "Timeout")
    end)

    {:noreply, _} = StashReceiver.handle_info(:cycle, opts)

    assert_receive :cycle, 1200
  end
end
