defmodule PoeSystem.Repo.Migrations.ChangeBuildsColumnTypes do
  use Ecto.Migration

  def change do
    execute("TRUNCATE TABLE builds", "")
    execute("ALTER TABLE builds DROP COLUMN provided", "")
    execute("ALTER TABLE builds DROP COLUMN found", "")
    execute("ALTER TABLE builds ADD COLUMN provided BYTEA NOT NULL", "")
    execute("ALTER TABLE builds ADD COLUMN found BYTEA", "")
  end
end
