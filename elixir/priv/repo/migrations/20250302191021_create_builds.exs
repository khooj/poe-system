defmodule PoeSystem.Repo.Migrations.CreateBuilds do
  use Ecto.Migration

  def change do
    create table(:builds, primary_key: false) do
      add :id, :uuid, primary_key: true
      add :data, :map, null: false
      add :processed, :boolean, default: false, null: false

      timestamps(type: :utc_datetime, inserted_at: :created_at)
    end
  end
end
