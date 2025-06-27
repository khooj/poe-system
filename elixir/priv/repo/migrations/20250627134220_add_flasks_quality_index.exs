defmodule PoeSystem.Repo.Migrations.AddFlasksQualityIndex do
  use Ecto.Migration

  def change do
    create index(:items, ["((info->>'quality')::int)"], where: "category = 'Flasks'")
  end
end
