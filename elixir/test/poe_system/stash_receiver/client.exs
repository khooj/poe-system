defmodule PoeSystem.StashReceiver.ClientTest do
  use PoeSystemWeb.ConnCase
  alias PoeSystem.StashReceiver.Client
  alias PoeSystem.Testdata

  test "check basic client working" do
    plug = {Req.Test, Client}

    Req.Test.stub(Client, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> send_resp(200, Testdata.stash_json())
    end)

    resp = Client.get_stash_data(nil, plug: plug, access_token: "empty")
    assert resp.body == Testdata.stash_json()
  end
end
