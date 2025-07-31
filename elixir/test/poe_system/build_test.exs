defmodule PoeSystem.BuildTest do
  alias PoeSystem.Testdata
  alias PoeSystem.{Build, Repo}
  alias Ecto.UUID
  use PoeSystem.DataCase
  use ExUnit.Case, async: true

  setup do
    build = Testdata.extract_config()
    build_nores = Testdata.extract_config(profile: "simplenores")
    :rarity
    %{build: build, build_nores: build_nores}
  end

  test "insert build (internal change)", %{build: build} do
    Build.internal_change(%{
      id: UUID.generate(),
      itemset: "test",
      skillset: "test",
      pob: Testdata.pobdata_file(),
      provided: build.provided
    })
    |> Repo.insert!()
  end
end
