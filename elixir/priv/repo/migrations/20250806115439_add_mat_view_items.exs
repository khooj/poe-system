defmodule PoeSystem.Repo.Migrations.AddMatViewItems do
  use Ecto.Migration

  def change do
    drop index(:items, ["((jsonb_path_query_array(info, '$.mods[*].stat_id')))"])
    execute(~S"
      CREATE MATERIALIZED VIEW items_mods AS select item_id, array(select jsonb_array_elements_text(jsonb_path_query_array(info, '$.mods[*].stat_id'))) as mods from items
      ", "DROP MATERIALIZED VIEW items_mods")
    create index("items_mods", ["((mods))"], using: "GIN")
    execute(~S"
      CREATE OR REPLACE FUNCTION refresh_items_mods()
      RETURNS trigger AS $$
      BEGIN
        REFRESH MATERIALIZED VIEW items_mods;
        RETURN NULL;
      END;
      $$ LANGUAGE plpgsql;
      ", "DROP FUNCTION refresh_items_mods()")
    execute(~S"
      CREATE TRIGGER refresh_items_mods_trg
      AFTER INSERT OR UPDATE OR DELETE
      ON items
      FOR EACH STATEMENT
      EXECUTE PROCEDURE refresh_items_mods();
      ", "DROP TRIGGER refresh_items_mods_trg")
  end
end
