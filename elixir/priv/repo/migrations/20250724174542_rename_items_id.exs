defmodule PoeSystem.Repo.Migrations.RenameItemsId do
  use Ecto.Migration

  def change do
    rename table(:items), :item_id, to: :item_id_tmp
    rename table(:items), :id, to: :item_id
    rename table(:items), :item_id_tmp, to: :id
  end
end
