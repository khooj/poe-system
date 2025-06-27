defmodule PoeSystem.Repo.Migrations.ChangeGemsFlasksIndexes do
  use Ecto.Migration

  def change do
    drop index(:items, ["((info->>'level')::int)"])
    drop index(:items, ["((info->>'quality')::int)"])
    create index(:items, ["basetype", "((info->>'level')::int)", "((info->>'quality')::int)"], where: "subcategory = 'Gem'")
  end
end
