defmodule PoeSystem.Repo.Migrations.AddBuildFixFlag do
  use Ecto.Migration

  def change do
    alter table(:builds) do
      add :fixed, :boolean, default: false, null: false
    end
  end
end
