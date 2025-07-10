defmodule PoeSystemWeb.Poe1Controller do
  alias Ecto.UUID
  alias PoeSystem.BuildProcessing
  alias PoeSystem.Build
  alias PoeSystem.Repo
  alias Ecto.Multi
  import Ecto.Query
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
              provided: config["provided"],
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
        "skillset" => skillset,
        "profile" => profile
      }) do
    {:ok, extracted_config} =
      RustPoe.Native.extract_build_config(pob_data, itemset, skillset, profile)

    conn
    |> json(%{config: extracted_config})
  end

  def set_profile(conn, %{"config" => cfg, "profile" => profile}) do
    {:ok, new_cfg} = RustPoe.Native.fill_configs_by_rule(cfg, profile)

    conn
    |> json(%{config: new_cfg})
  end

  def get_build(conn, %{"id" => id}) do
    build =
      Repo.one(
        from b in Build, where: b.id == ^id, select: %{fixed: b.fixed, processed: b.processed}
      )

    case build do
      %{fixed: true} = data ->
        conn
        |> assign_prop(:provided, fn -> Build.get_provided(id) end)
        |> assign_prop(:found, fn -> Build.get_found(id) end)
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
