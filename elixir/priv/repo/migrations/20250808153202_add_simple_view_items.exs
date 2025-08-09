defmodule PoeSystem.Repo.Migrations.AddSimpleViewItems do
  use Ecto.Migration

  def change do
    execute("DROP TRIGGER refresh_items_mods_trg on items", "")
    execute("DROP FUNCTION refresh_items_mods()", "")
    execute(~S"DROP MATERIALIZED VIEW items_mods", "DROP MATERIALIZED VIEW items_mods")
    create index(:items, ["jsonb_path_query_array(info, '$.mods[*].stat_id') jsonb_path_ops"], using: "GIN")
  end
end
