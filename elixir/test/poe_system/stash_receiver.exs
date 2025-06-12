defmodule PoeSystem.StashReceiverTest do
  use ExUnit.Case
  use PoeSystemWeb.ConnCase
  alias PoeSystem.StashReceiver.Client
  alias PoeSystem.StashReceiver
  alias PoeSystem.Testdata
  alias Ecto.Multi
  alias PoeSystem.Repo

  test "process stash data from api" do
    resp =
      Req.Response.new(
        status: 200,
        body: Testdata.stash_json(),
        headers: %{
          content_type: "application/json"
        }
      )

    multi = StashReceiver.receive_stash(resp, nil)

    assert {:error, :success} =
             Repo.transaction(fn ->
               Repo.transaction(multi)
               Repo.rollback(:success)
             end)
  end
end
