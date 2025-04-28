defmodule PoeSystem.Repo.Migrations.AddItemRarity do
  use Ecto.Migration

  def change do
    alter table(:items) do
      add :rarity, :text, null: false
    end
  end
end
