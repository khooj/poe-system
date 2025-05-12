defmodule PoeSystemWeb.Poe1Controller do
  alias Ecto.UUID
  alias PoeSystem.BuildProcessing
  alias PoeSystem.BuildInfo
  alias PoeSystem.Repo
  alias Ecto.Multi
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    build_ids = BuildInfo.get_ids()

    conn
    |> assign_prop(:build_ids, build_ids)
    |> render_inertia("poe1/Index")
  end

  def new(conn, %{"id" => id}) do
    build = BuildInfo.get_build(id)

    Multi.new()
    |> Multi.update(:update, BuildInfo.changeset(build, %{fixed: true}))
    |> BuildProcessing.queue_processing_build_multi(:new_job, id)
    |> PoeSystem.Repo.transaction()

    conn
    |> redirect(to: ~p"/poe1/build/#{id}")
  end

  def extract(conn, %{
        "itemset" => itemset,
        "pobData" => pob_data,
        "skillset" => skillset
      }) do
    {:ok, extracted_config} = RustPoe.Native.extract_build_config(pob_data, itemset, skillset)

    {:ok, data} =
      BuildInfo.changeset(%BuildInfo{}, %{
        id: UUID.bingenerate(),
        data: extracted_config,
        itemset: itemset,
        skillset: skillset,
        pob: pob_data
      })
      |> Repo.insert()

    conn
    |> redirect(to: ~p"/poe1/preview/#{data.id}")
  end

  def preview(conn, %{"id" => id}) do
    case BuildInfo.get_build(id) do
      nil ->
        conn
        |> put_flash(:info, "Build does not exist")
        |> put_status(404)
        |> redirect(to: ~p"/poe1")

      %{fixed: true} ->
        conn
        |> put_flash(:info, "Build already configured")
        |> redirect(to: ~p"/poe1/build/#{id}")

      build ->
        conn
        |> assign_prop(:build_data, build)
        |> render_inertia("poe1/Preview")
    end
  end

  def update_build_config(conn, %{"id" => id, "config" => config}) do
    build = BuildInfo.get_build(id)

    if build.fixed do
      conn
      |> put_flash(:info, "Build config cannot be updated")
      |> redirect(to: ~p"/poe1/build/#{id}")
    else
      {:ok, validated_config} =
        RustPoe.Native.validate_and_apply_config(build.data, config)

      {:ok, _} =
        BuildInfo.changeset(build, %{
          data: validated_config
        })
        |> Repo.update()

      conn
      |> redirect(to: ~p"/poe1/preview/#{id}")
    end
  end

  def get_build(conn, %{"id" => id}) do
    case BuildInfo.get_build(id) do
      nil ->
        conn
        |> put_flash(:info, "Build does not exist")
        |> put_status(404)
        |> redirect(to: ~p"/poe1")

      %{fixed: true} = data ->
        conn
        |> assign_prop(:data, data.data)
        |> assign_prop(:processed, data.processed)
        |> render_inertia("poe1/Build")

      _ ->
        conn
        |> put_flash(:info, "Build not configured")
        |> redirect(to: ~p"/poe1/preview/#{id}")
    end
  end
end
