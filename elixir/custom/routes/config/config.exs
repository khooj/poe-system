import Config

if Mix.env() == :test do
  config :routes,
    routes_path: "test/support"
end
