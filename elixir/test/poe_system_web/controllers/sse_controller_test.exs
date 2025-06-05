defmodule PoeSystemWeb.SseControllerTest do
  use PoeSystemWeb.ConnCase
  alias Phoenix.PubSub
  alias Ecto.UUID
  alias PoeSystem.Testdata

  @moduletag timeout: :timer.seconds(1)

  # FIXME: somehow get message from conn
  @tag :skip
  test "subscribe to build notification", %{conn: conn} do
    # topic = "build:#{UUID.load!(UUID.bingenerate())}"
    #
    # t =
    #   Task.async(fn ->
    #     conn = post(conn, ~p"/poe1/sse", topics: topic)
    #   end)
    #
    # PubSub.broadcast!(PoeSystem.PubSub, topic, {PoeSystem.PubSub, "test"})
    # IO.inspect(Process.info(self(), :messages))
    # assert_receive s
    # assert Task.await(t)
  end

  test "subscribe failure to incorrect or not allowed topic", %{conn: conn} do
    conn = post(conn, ~p"/poe1/sse", topics: "invalid-topic")
    assert response(conn, 400)
  end
end
