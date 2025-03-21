defmodule PoeSystem.BuildInfoTest do
  alias PoeSystem.BuildInfo
  use PoeSystem.DataCase

  setup do
    BuildInfo.add_build(%{provided: nil, found: nil})
    :ok
  end

  test "get build info" do
    infos = Repo.all(BuildInfo)
    assert length(infos) > 0
  end
end
