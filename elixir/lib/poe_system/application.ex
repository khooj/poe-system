defmodule PoeSystem.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    :ok = Oban.Telemetry.attach_default_logger()
    :ok = OpentelemetryBandit.setup()
    :ok = OpentelemetryPhoenix.setup(adapter: :bandit)
    :ok = OpentelemetryEcto.setup([:poe_system, :repo])
    :ok = OpentelemetryOban.setup()

    children =
      [
        PoeSystemWeb.PromEx,
        # PoeSystemWeb.Telemetry,
        PoeSystem.Repo,
        {DNSCluster, query: Application.get_env(:poe_system, :dns_cluster_query) || :ignore},
        {Phoenix.PubSub, name: PoeSystem.PubSub},
        {Oban, Application.fetch_env!(:poe_system, Oban)},
        # Start a worker by calling: PoeSystem.Worker.start_link(arg)
        # {PoeSystem.Worker, arg},
        {Inertia.SSR,
         path: Path.join([Application.app_dir(:poe_system), "priv/static/assets/ssr"]),
         module: "ssr.mjs"},
        {PoeSystemWeb.RateLimit, clean_period: :timer.minutes(10)},
        # Start to serve requests, typically the last entry
        PoeSystemWeb.Endpoint
      ] ++ Application.get_env(:poe_system, :additional_processes, [])

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: PoeSystem.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    PoeSystemWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
