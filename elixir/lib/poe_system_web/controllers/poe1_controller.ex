defmodule PoeSystemWeb.Poe1Controller do
  alias Ecto.UUID
  alias PoeSystem.BuildProcessing
  alias PoeSystem.Build
  alias Ecto.Multi
  alias PoeSystem.RateLimit
  use PoeSystemWeb, :controller
  use Telemetria

  @ratelimit_opts %{
    time_window: :timer.seconds(1),
    limit: 2
  }

  @telemetria level: :info, group: :poe1_build_cost_calc
  def index(conn, _params) do
    conn
    |> render_inertia("poe1/Index")
  end

  def new(conn, %{
        "config" => config,
        "itemset" => itemset,
        "pobData" => pob_data,
        "skillset" => skillset
      }) do
    ip = :inet.ntoa(conn.remote_ip)

    case RateLimit.hit(
           "#{ip}_new",
           @ratelimit_opts.time_window,
           @ratelimit_opts.limit
         ) do
      {:allow, _} ->
        :ok = RustPoe.Native.validate_config(config)

        {:ok, ret} =
          Multi.new()
          |> Multi.insert(
            :bi,
            Build.changeset(%Build{}, %{
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
        |> redirect(to: ~p"/poe1/build-calc/#{ret.bi.id}")

      {:deny, _} ->
        conn
        |> send_resp(429, "Too Many Requests")
    end
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
    case Build.get_build(id) do
      %{fixed: true} = data ->
        conn
        # TODO: maybe use partial reload?
        |> assign_prop(:provided, data.data["provided"])
        |> assign_prop(:found, data.data["found"])
        |> assign_prop(:processed, data.processed)
        |> assign_prop(:id, id)
        |> render_inertia("poe1/Build")

      _ ->
        conn
        |> put_flash(:info, "Build does not exist")
        |> redirect(to: ~p"/poe1/build-calc")
    end
  end
end
