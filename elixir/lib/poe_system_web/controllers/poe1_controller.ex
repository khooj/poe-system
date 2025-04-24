defmodule PoeSystemWeb.Poe1Controller do
  alias PoeSystem.BuildProcessing
  alias PoeSystem.BuildInfoPreview
  alias PoeSystem.BuildInfo
  alias Ecto.Multi
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    build_ids = BuildInfo.get_ids()

    conn
    |> assign_prop(:build_ids, build_ids)
    |> render_inertia("poe1/Index")
  end

  def new(conn, %{"id" => id}) do
    build = BuildInfoPreview.get_build(id)

    Multi.new()
    |> Multi.insert(:insert, BuildInfo.add_build_changeset(id, build.data))
    |> Multi.delete(:delete, %BuildInfoPreview{id: id})
    |> BuildProcessing.queue_processing_build_multi(:new_job, id)
    |> PoeSystem.Repo.transaction()

    # {:ok, _} = BuildInfo.add_build(id, build.data)
    # {:ok, _} = BuildInfoPreview.remove(%BuildInfoPreview{id: id})

    conn
    |> redirect(to: ~p"/poe1/build/#{id}")
  end

  def extract(conn, %{
        "itemset" => itemset,
        "pobData" => pob_data,
        "skillset" => skillset
      }) do
    {:ok, extracted_config} = RustPoe.Native.extract_build_config(pob_data, itemset, skillset)
    {:ok, data} = BuildInfoPreview.add_build(extracted_config, itemset, skillset, pob_data)

    conn
    |> redirect(to: ~p"/poe1/preview/#{data.id}")
  end

  def preview(conn, %{"id" => id}) do
    data = BuildInfoPreview.get_build(id)

    conn
    |> assign_prop(:build_data, data)
    |> render_inertia("poe1/Preview")
  end

  def update_preview(conn, %{"id" => id, "config" => config}) do
    build = BuildInfoPreview.get_build(id)

    {:ok, validated_config} =
      RustPoe.Native.validate_and_apply_config(build.data, config)

    {:ok, _} = BuildInfoPreview.update(build, validated_config)

    conn
    |> redirect(to: ~p"/poe1/preview/#{id}")
  end

  def get_build(conn, %{"id" => id}) do
    data = BuildInfo.get_build(id)

    conn
    |> assign_prop(:data, data.data)
    |> render_inertia("poe1/Build")
  end
end
