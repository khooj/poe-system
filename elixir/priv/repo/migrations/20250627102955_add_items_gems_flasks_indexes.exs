defmodule PoeSystem.Repo.Migrations.AddItemsGemsFlasksIndexes do
  use Ecto.Migration

  def change do
    create index(:items, ["((info->>'level')::int)"])
    create index(:items, ["((info->>'quality')::int)"])
  end
end
