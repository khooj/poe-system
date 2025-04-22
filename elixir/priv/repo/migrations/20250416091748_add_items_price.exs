defmodule PoeSystem.Repo.Migrations.AddItemsPrice do
  use Ecto.Migration

  def change do
    alter table(:items) do
      add :price, :map, null: false
    end
  end
end
