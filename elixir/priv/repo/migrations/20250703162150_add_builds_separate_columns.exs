defmodule PoeSystem.Repo.Migrations.AddBuildsSeparateColumns do
  use Ecto.Migration

  def change do
    alter table(:builds) do
      add :provided, :map
      add :found, :map
    end

    execute("UPDATE builds SET provided = data->'provided', found = data->'found'", "")
    execute("ALTER TABLE builds ALTER COLUMN provided SET NOT NULL", "")
    execute("ALTER TABLE builds DROP COLUMN data", "")
  end
end
