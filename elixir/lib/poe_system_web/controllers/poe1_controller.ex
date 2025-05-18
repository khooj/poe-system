defmodule PoeSystemWeb.Poe1Controller do
  alias Ecto.UUID
  alias PoeSystem.BuildProcessing
  alias PoeSystem.BuildInfo
  alias Ecto.Multi
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    build_ids = BuildInfo.get_ids()

    conn
    |> assign_prop(:build_ids, build_ids)
    |> render_inertia("poe1/Index")
  end

  def new(conn, %{
        "config" => config,
        "itemset" => itemset,
        "pobData" => pob_data,
        "skillset" => skillset
      }) do
    :ok = RustPoe.Native.validate_config(config)

    {:ok, ret} =
      Multi.new()
      |> Multi.insert(
        :bi,
        BuildInfo.changeset(%BuildInfo{}, %{
          id: UUID.bingenerate(),
          data: config,
          itemset: itemset,
          skillset: skillset,
          pob: pob_data,
          fixed: true
        })
      )
      |> BuildProcessing.queue_processing_build_multi(:new_job, fn %{bi: bi} ->
        BuildProcessing.new(%{id: bi.id})
      end)
      |> PoeSystem.Repo.transaction()

    conn
    |> redirect(to: ~p"/poe1/build/#{ret.bi.id}")
  end

  def extract(conn, %{
        "itemset" => itemset,
        "pobData" => pob_data,
        "skillset" => skillset
      }) do
    {:ok, extracted_config} = RustPoe.Native.extract_build_config(pob_data, itemset, skillset)

    conn
    |> json(%{config: extracted_config})
  end

  def get_build(conn, %{"id" => id}) do
    case BuildInfo.get_build(id) do
      %{fixed: true} = data ->
        conn
        |> assign_prop(:data, data.data)
        |> assign_prop(:processed, data.processed)
        |> render_inertia("poe1/Build")

      _ ->
        conn
        |> put_flash(:info, "Build does not exist")
        |> put_status(404)
        |> redirect(to: ~p"/poe1")
    end
  end
end
