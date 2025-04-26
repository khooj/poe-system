defmodule PoeSystem.Repo.Migrations.RemoveNewBuildsQueue do
  use Ecto.Migration

  def change do
    drop table(:new_builds)

    execute(
      "DROP TRIGGER new_builds ON builds;",
      ~S"""
        CREATE OR REPLACE TRIGGER new_builds AFTER INSERT ON builds
        FOR EACH ROW EXECUTE FUNCTION new_build_added();
      """
    )

    execute(
      "DROP FUNCTION new_build_added;",
      ~S"""
      CREATE OR REPLACE FUNCTION new_build_added() RETURNS TRIGGER AS $$
        BEGIN
          INSERT INTO new_builds(build_id) VALUES (NEW.id);
          RETURN NULL;
        END;
      $$ LANGUAGE plpgsql;
      """
    )
  end
end
