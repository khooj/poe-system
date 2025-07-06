defmodule PoeSystem.BuildProcessingTest do
  alias PoeSystem.Testdata
  alias PoeSystem.BuildProcessing
  use PoeSystem.DataCase
  use ExUnit.Case, async: true

  setup do
    build = Testdata.extract_config()
    %{build: build}
  end

  test "basic queue check" do
    assert {:ok, _} = BuildProcessing.queue_processing_build("testid")
    assert_enqueued(worker: BuildProcessing, args: %{id: "testid"})
  end

  test "basic queue check multi" do
    assert {:ok, _} =
             Ecto.Multi.new()
             |> BuildProcessing.queue_processing_build_multi(:new_job, fn _ ->
               BuildProcessing.new(%{id: "testid"})
             end)
             |> Repo.transaction()

    assert_enqueued(worker: BuildProcessing, args: %{id: "testid"})
  end

  describe "process single build" do
    setup do
      Testdata.insert_items()
      Testdata.insert_build()
      :ok
    end

    test "w/ items", %{build: build} do
      assert BuildProcessing.process_single_build(build["provided"])
    end

    # TODO: ensure that testdata have required items
    @tag :skip
    test "check gems", %{build: build} do
      assert not Enum.empty?(build["provided"]["gems"])
      processed = BuildProcessing.process_single_build(build)
      assert not Enum.empty?(processed["found"]["gems"])
    end

    @tag :skip
    test "check flasks", %{build: build} do
      assert not Enum.empty?(build["provided"]["flasks"])
      processed = BuildProcessing.process_single_build(build)
      assert not Enum.empty?(processed["found"]["flasks"])
    end
  end
end
