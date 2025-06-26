defmodule PoeSystem.Repo.Migrations.ChangeItemsIndexes do
  use Ecto.Migration

  def change do
    drop index(:items, ["(data -> 'mods') jsonb_path_ops"])
    create index(:items, ["(info->'mods'->'stat_id')"], using: "GIN")
  end
end
