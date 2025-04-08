defmodule PoeSystem.Repo.Migrations.CreateBuilds do
  use Ecto.Migration

  def change do
    create table(:builds, primary_key: false) do
      add :id, :uuid, primary_key: true
      add :data, :map, null: false
      add :processed, :boolean, default: false, null: false

      timestamps(type: :timestamptz)
    end

    create table(:build_info_previews, primary_key: false) do
      add :id, :uuid, primary_key: true
      add :data, :map, null: false
      add :itemset, :text, null: false
      add :skillset, :text, null: false
      add :pob, :text, null: false

      timestamps(type: :timestamptz)
    end

    create table(:stashes, primary_key: false) do
      add :id, :text
      add :item_id, :text, null: false
    end

    create index(:stashes, [:id, :item_id])

    create table(:items, primary_key: false) do
      add :id, :text, primary_key: true
      add :data, :map, null: false
      add :basetype, :text, null: false
      add :category, :text, null: false
      add :subcategory, :text, null: false
      add :name, :text, null: false
    end

    create index(:items, ["(data -> 'mods')"], using: "GIN")
    create index(:items, [:category], using: "BTREE")
    create index(:items, [:subcategory], using: "BTREE")

    create table(:latest_stash, primary_key: false) do
      add :id, :text, primary_key: true
    end

    create table(:new_builds) do
      add :build_id, references(:builds, type: :uuid), null: false
      add :processing, :boolean, null: false, default: false
      add :started_at, :timestamptz
    end

    execute(
      ~S"""
      CREATE OR REPLACE FUNCTION new_build_added() RETURNS TRIGGER AS $$
        BEGIN
          INSERT INTO new_builds(build_id) VALUES (NEW.id);
          RETURN NULL;
        END;
      $$ LANGUAGE plpgsql;
      """,
      "DROP FUNCTION new_build_added;"
    )

    execute(
      ~S"""
        CREATE OR REPLACE TRIGGER new_builds AFTER INSERT ON builds
        FOR EACH ROW EXECUTE FUNCTION new_build_added();
      """,
      "DROP TRIGGER new_builds;"
    )
  end
end
