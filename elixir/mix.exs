defmodule PoeSystem.MixProject do
  use Mix.Project

  def project do
    [
      app: :poe_system,
      version: "0.1.0",
      elixir: "~> 1.18",
      elixirc_paths: elixirc_paths(Mix.env()),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      compilers: [:telemetria | Mix.compilers()],
      releases: [
        poe_system: [
          applications: [
            poe_system: :permanent,
            # TODO: enable for traces and metrics
            opentelemetry_exporter: :permanent,
            opentelemetry: :temporary
          ]
        ]
      ],
      dialyzer: [
        plt_add_apps: [:mix]
      ]
    ]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [
      mod: {PoeSystem.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [
      {:phoenix, "~> 1.7.19"},
      {:phoenix_ecto, "~> 4.5"},
      {:ecto_sql, "~> 3.10"},
      {:postgrex, ">= 0.0.0"},
      {:phoenix_html, "~> 4.1"},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:phoenix_live_view, "~> 1.0.0"},
      {:floki, ">= 0.30.0", only: :test},
      {:phoenix_live_dashboard, "~> 0.8.3"},
      {:telemetry_metrics, "~> 1.0"},
      {:telemetry_poller, "~> 1.0"},
      {:gettext, "~> 0.26"},
      {:jason, "~> 1.2"},
      {:dns_cluster, "~> 0.1.1"},
      {:bandit, "~> 1.5"},
      {:inertia, "~> 2.2.0"},
      {:rustler, "~> 0.36.1", runtime: false},
      {:routes, path: "custom/routes"},
      {:nodejs, path: "custom/elixir-nodejs", override: true},
      {:oban, "~> 2.19.4"},
      {:hammer, "~> 7.0"},
      {:telemetria, "~> 0.1"},
      {:opentelemetry_api, "~> 1.4"},
      {:opentelemetry_exporter, "~> 1.8"},
      {:opentelemetry, "~> 1.5"},
      {:opentelemetry_phoenix, "~> 2.0"},
      {:opentelemetry_bandit, "~> 0.2"},
      {:opentelemetry_ecto, "~> 1.2"},
      # FIXME: use official package after oban semconv changes merge
      {:opentelemetry_oban,
       git: "https://github.com/danschultzer/opentelemetry-erlang-contrib",
       branch: "update-oban-attributes",
       sparse: "instrumentation/opentelemetry_oban"},
      {:dialyxir, "~> 1.4", only: [:dev, :test], runtime: false},
      {:mix_audit, "~> 2.1", only: [:dev, :test], runtime: false},
      {:sobelow, "~> 0.14", only: [:dev, :test], runtime: false},
      {:dotenvy, "~> 1.0.0"},
      {:prom_ex, "~> 1.11"},
      {:plug, "~> 1.0"},
      {:plug_cowboy, "~> 2.0"},
      {:sse_phoenix_pubsub, "~> 1.0"},
      {:req, "~> 0.5"},
      {:websockex, "~> 0.4.3", only: [:dev, :test]},
      {:broadway, "~> 1.2"},
      {:flow, "~> 1.2"},
      {:nimble_options, "~> 1.1"},
      {:nimble_parsec, "~> 1.4"}
    ]
  end

  # Aliases are shortcuts or tasks specific to the current project.
  # For example, to install project dependencies and perform other setup tasks, run:
  #
  #     $ mix setup
  #
  # See the documentation for `Mix` for more info on aliases.
  defp aliases do
    [
      setup: ["deps.get", "ecto.setup"],
      deps: ["deps.get", "deps.compile"],
      "ecto.setup": ["ecto.create", "ecto.migrate", "run priv/repo/seeds.exs"],
      "ecto.reset": ["ecto.drop", "ecto.setup"],
      test: ["ecto.create --quiet", "ecto.migrate --quiet", "test"],
      "assets.build": ["npm"]
    ]
  end
end
