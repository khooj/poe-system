defmodule PoeSystem.StashReceiver.ClientTest do
  use PoeSystemWeb.ConnCase
  alias PoeSystem.StashReceiver.Client
  alias PoeSystem.Testdata

  setup do
    %{
      opts: %{
        plug: {Req.Test, Client},
        access_token: "empty"
      }
    }
  end

  test "basic client working", %{opts: opts} do
    Req.Test.stub(Client, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> send_resp(200, Testdata.stash_json())
    end)

    resp = Client.get_stash_data(nil, opts)
    assert resp.body == Testdata.stash_json()
  end
end
