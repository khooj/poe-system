defmodule PoeSystemWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :poe_system

  # The session will be stored in the cookie and signed,
  # this means its contents can be read but not tampered with.
  # Set :encryption_salt if you would also like to encrypt it.
  @session_options [
    store: :cookie,
    key: "_poe_system_key",
    signing_salt: "4X8fx9Ow",
    encryption_salt: "mLQTvgj+",
    same_site: "Lax",
    http_only: true,
    max_age: 86400
  ]

  # socket "/live", Phoenix.LiveView.Socket,
  #   websocket: [connect_info: [session: @session_options]],
  #   longpoll: [connect_info: [session: @session_options]]

  # Serve at "/" the static files from "priv/static" directory.
  #
  # You should set gzip to true if you are running phx.digest
  # when deploying your static files in production.
  #

  if Application.compile_env!(:poe_system, :mode) == :dev do
    plug PoeSystemWeb.Plug.Proxy,
      match_path: "/assets",
      redirect_to: "http://localhost:5173"
  else
    plug Plug.Static,
      at: "/",
      from: :poe_system,
      gzip: false,
      only: PoeSystemWeb.static_paths(),
      cache_control_for_etags: "public,max-age=31536000,immutable"
  end

  # Code reloading can be explicitly enabled under the
  # :code_reloader configuration of your endpoint.
  if code_reloading? do
    socket "/phoenix/live_reload/socket", Phoenix.LiveReloader.Socket
    plug Phoenix.LiveReloader
    plug Phoenix.CodeReloader
    plug Phoenix.Ecto.CheckRepoStatus, otp_app: :poe_system
  end

  # plug Phoenix.LiveDashboard.RequestLogger,
  #   param_key: "request_logger",
  #   cookie_key: "request_logger"

  plug Plug.RequestId
  plug Plug.Telemetry, event_prefix: [:phoenix, :endpoint]

  plug Plug.Parsers,
    parsers: [:urlencoded, :multipart, :json],
    pass: ["application/json", "multipart/form-data", "application/x-www-form-urlencoded"],
    json_decoder: Phoenix.json_library()

  plug Plug.MethodOverride
  plug Plug.Head
  # X-Forwarded-For header should be set correctly on proxy
  plug Plug.RewriteOn, [
    :x_forwarded_for,
    :x_forwarded_host,
    :x_forwarded_port,
    :x_forwarded_proto
  ]

  plug Plug.Session, @session_options
  plug PoeSystemWeb.Router
end
