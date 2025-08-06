defmodule PoeSystem.Repo.Migrations.ChangeItemsIndexesAgain2 do
  use Ecto.Migration

  def change do
    drop index(:items, ["basetype", "((info->>'level')::int)", "((info->>'quality')::int)"])
    drop index(:items, ["(info->'mods'->'stat_id')"])
    create index(:items, ["((jsonb_path_query_array(info, '$.mods[*].stat_id')))"], using: "GIN")
  end
end
