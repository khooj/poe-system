defmodule PoeSystem.BuildInfoTest do
  alias PoeSystem.BuildInfo
  use PoeSystem.DataCase

  test "get build info" do
    infos = Repo.all(BuildInfo)
    assert length(infos) > 0
  end
end
