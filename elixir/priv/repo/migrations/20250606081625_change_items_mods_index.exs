defmodule PoeSystem.Repo.Migrations.ChangeItemsModsIndex do
  use Ecto.Migration

  # TODO: benchmark index usage
  def change do
    drop index(:items, ["(data -> 'mods')"], using: "GIN")
    create index(:items, ["(data -> 'mods') jsonb_path_ops"], using: "GIN")
  end
end
