# This file is responsible for configuring your application
# and its dependencies with the aid of the Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
import Config

config :poe_system,
  ecto_repos: [PoeSystem.Repo],
  generators: [timestamp_type: :utc_datetime]

# Configures the endpoint
config :poe_system, PoeSystemWeb.Endpoint,
  url: [host: "localhost"],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: PoeSystemWeb.ErrorHTML, json: PoeSystemWeb.ErrorJSON],
    layout: false
  ],
  pubsub_server: PoeSystem.PubSub,
  live_view: [signing_salt: "W/E8aK8j"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

config :inertia,
  endpoint: PoeSystemWeb.Endpoint,
  static_paths: ["/assets/main.js"],
  camelize_props: false,
  ssr: false,
  raise_on_ssr_failure: config_env() != :prod

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

config :routes,
  router: PoeSystemWeb.Router,
  typescript: true,
  routes_path: "assets/src"

config :nodejs,
  executable: "bun"

config :poe_system, Oban,
  repo: PoeSystem.Repo,
  engine: Oban.Engines.Basic,
  queues: [new_builds: 1]

config :telemetria,
  backend: Telemetria.Backend.OpenTelemetry,
  otp_app: :poe_system,
  purge_level: :debug,
  level: :info

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{config_env()}.exs"
