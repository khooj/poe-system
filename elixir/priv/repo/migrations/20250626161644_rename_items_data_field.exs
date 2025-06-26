defmodule PoeSystem.Repo.Migrations.RenameItemsDataField do
  use Ecto.Migration

  def change do
    execute("ALTER TABLE items RENAME COLUMN data TO info", "")
  end
end
