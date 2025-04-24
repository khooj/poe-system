import Config

# Configure your database
#
# The MIX_TEST_PARTITION environment variable can be used
# to provide built-in test partitioning in CI environment.
# Run `mix help test` for more information.
config :poe_system, PoeSystem.Repo,
  username: "khooj",
  password: "khooj",
  hostname: "localhost",
  database: "poe_system_test#{System.get_env("MIX_TEST_PARTITION")}",
  pool: Ecto.Adapters.SQL.Sandbox,
  pool_size: System.schedulers_online() * 2

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :poe_system, PoeSystemWeb.Endpoint,
  http: [ip: {127, 0, 0, 1}, port: 4002],
  secret_key_base: "DPqukG08gn2vW7GBXk8Ca6pAV0dvlVcUqnK98/nY1Yqo7LYEfE1sff0EnT1VSLke",
  server: false

config :inertia,
  endpoint: PoeSystemWeb.Endpoint,
  static_paths: ["/assets/main.js"],
  camelize_props: true,
  ssr: false,
  raise_on_ssr_failure: config_env() != :prod

# Print only warnings and errors during test
config :logger, level: :warning

# Initialize plugs at runtime for faster test compilation
config :phoenix, :plug_init_mode, :runtime

config :poe_system, Oban, testing: :manual

# Enable helpful, but potentially expensive runtime checks
config :phoenix_live_view,
  enable_expensive_runtime_checks: true
