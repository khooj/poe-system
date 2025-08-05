defmodule PoeSystem.Repo.Migrations.ChangeItemsIndexesAgain do
  use Ecto.Migration

  def change do
    drop index(:items, ["((info->>'quality')::int)"])
    drop index(:items, ["basetype", "((info->>'level')::int)", "((info->>'quality')::int))"])
    create index(:items, ["((info->>'quality')::int)"], where: "category = 'flasks'")
    create index(:items, ["basetype", "((info->>'level')::int)", "((info->>'quality')::int)"], where: "subcategory = 'gem'")
  end
end
