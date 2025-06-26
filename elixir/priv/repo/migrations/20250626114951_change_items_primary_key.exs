defmodule PoeSystem.Repo.Migrations.ChangeItemsPrimaryKey do
  use Ecto.Migration

  def change do
      execute("ALTER TABLE items RENAME COLUMN id TO item_id;", "")
      execute("ALTER TABLE items DROP CONSTRAINT items_pkey;", "")
      execute("ALTER TABLE items ADD CONSTRAINT items_item_id_uniq UNIQUE (item_id);", "")
      execute("ALTER TABLE items ADD COLUMN id SERIAL PRIMARY KEY;", "")
  end
end
