defmodule PoeSystem.StashReceiverTest do
  use PoeSystemWeb.ConnCase

  alias PoeSystem.StashReceiver

  test "test message" do
    ref = Broadway.test_message(StashReceiverImpl, 1)
    assert_receive {:ack, ^ref, [%{data: 1, metadata: %{check: true}}], []}
  end
end
