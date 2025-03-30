defmodule PoeSystem.Repo.Migrations.CreateBuildInfoPreviews do
  use Ecto.Migration

  def change do
    create table(:build_info_previews, primary_key: false) do
      add :id, :uuid, primary_key: true
      add :data, :map, null: false
      add :itemset, :text, null: false
      add :skillset, :text, null: false
      add :pob, :text, null: false

      timestamps(type: :utc_datetime)
    end
  end
end
