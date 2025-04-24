defmodule PoeSystem.BuildProcessingTest do
  alias PoeSystem.BuildProcessing
  use PoeSystem.DataCase

  test "basic queue check" do
    assert {:ok, _} = BuildProcessing.queue_processing_build("testid")
    assert_enqueued(worker: BuildProcessing, args: %{id: "testid"})
  end

  test "basic queue check multi" do
    assert {:ok, _} =
             Ecto.Multi.new()
             |> BuildProcessing.queue_processing_build_multi(:new_job, "testid")
             |> Repo.transaction()

    assert_enqueued(worker: BuildProcessing, args: %{id: "testid"})
  end
end
