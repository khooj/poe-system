defmodule PoeSystemWeb.Plug.ProxyTest do
  use PoeSystemWeb.ConnCase
  use ExUnit.Case, async: true
  import Plug.Test
  alias PoeSystemWeb.Plug.Proxy

  test "check proxy redirect work" do
    opts =
      Proxy.init(
        match_path: "/test/",
        redirect_to: "http://example.local:1234/red",
        plug: {Req.Test, PoeSystemWeb.Plug.Proxy}
      )

    Req.Test.stub(PoeSystemWeb.Plug.Proxy, fn conn ->
      Req.Test.json(conn, %{response: true})
    end)

    conn = conn(:get, "/test/asset.js")
    conn = Proxy.call(conn, opts)
    assert conn.resp_body == ~s|{"response":true}|
  end

  test "check proxy passthrough work" do
    opts =
      Proxy.init(
        match_path: "/test/",
        redirect_to: "http://example.local:1234/red"
      )

    conn = conn(:get, "/nottest/asset.js")
    conn2 = Proxy.call(conn, opts)
    assert conn == conn2
  end
end
