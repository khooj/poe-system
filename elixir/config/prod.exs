import Config

# Note we also include the path to a cache manifest
# containing the digested version of static files. This
# manifest is generated by the `mix assets.deploy` task,
# which you should run after static files are built and
# before starting your production server.
# config :poe_system, PoeSystemWeb.Endpoint,
#   cache_static_manifest: "priv/static/assets/cache_manifest.json"

# Do not print debug messages in production
config :logger, level: :info

config :poe_system, Rustler, skip_compilation?: true

config :poe_system, Oban,
  plugins: [
    # 7 days
    {Oban.Plugins.Pruner, max_age: 7 * 24 * 60 * 60},
    {Oban.Plugins.Lifeline, rescue_after: :timer.minutes(30)}
  ]

# Runtime production configuration, including reading
# of environment variables, is done on config/runtime.exs.
