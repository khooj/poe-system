defmodule PoeSystem do
  @moduledoc """
  PoeSystem keeps the contexts that define your domain
  and business logic.

  Contexts are also responsible for managing your data, regardless
  if it comes from the database, an external API or others.
  """

    alias PoeSystem.BuildInfo
    alias PoeSystem.Repo

    def add_build(attrs) do
      BuildInfo.changeset(%BuildInfo{}, attrs)
      |> Repo.insert()
    end
end
