defmodule PoeSystem.Repo.Migrations.RemoveBuildPreview do
  use Ecto.Migration

  def change do
    drop table(:build_info_previews)

    execute("TRUNCATE TABLE builds;", "")

    alter table(:builds) do
      add :itemset, :text, null: false
      add :skillset, :text, null: false
      add :pob, :text, null: false
    end
  end
end
