defmodule PoeSystemWeb.Router do
  use PoeSystemWeb, :router
  import PhoenixStorybook.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug PoeSystemWeb.Plug.RateLimiter
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {PoeSystemWeb.Layouts, :root}
    plug :protect_from_forgery
    # Config.CSP should be in sobelow-skips
    plug :put_secure_browser_headers

    if Application.compile_env!(:poe_system, :mode) == :prod do
      plug :put_content_security_policy,
           [
             default_src: "'self'",
             script_src: "'self' 'wasm-unsafe-eval' 'nonce'",
             style_src: "'self' 'nonce' https://cdnjs.cloudflare.com",
             img_src: "'self' data:"
           ] ++ Application.compile_env(:poe_system, :additional_csp, [])
    end

    plug Inertia.Plug
  end

  pipeline :api do
    plug :accepts, ["json"]
    plug PoeSystemWeb.Plug.RateLimiter
  end

  # Config.CSRF sse does not receive any msg so should be safe here
  pipeline :sse do
    plug :put_format, "text/event-stream"
    plug PoeSystemWeb.Plug.RateLimiter
    plug :fetch_session
  end

  scope "/" do
    storybook_assets()
  end

  scope "/", PoeSystemWeb, as: :main do
    pipe_through :browser

    get "/", IndexController, :index
    live "/test", TestLive
    live_storybook("/storybook", backend_module: PoeSystemWeb.Storybook)
  end

  scope "/poe1", PoeSystemWeb, as: :poe1 do
    pipe_through :browser

    scope "/build-calc", as: :build_calc do
      live "/", Poe1BuildCalcIndexLive
      post "/new", Poe1Controller, :new
      get "/:id", Poe1Controller, :get_build
    end
  end

  scope "/sse", PoeSystemWeb, as: :sse do
    pipe_through :sse

    post "/", SseController, :subscribe
  end

  # Other scopes may use custom stacks.
  scope "/api", PoeSystemWeb, as: :api do
    pipe_through :api

    scope "/v1", as: :v1 do
      post "/extract", Poe1Controller, :extract
      post "/profile", Poe1Controller, :set_profile
    end
  end

  # Enable LiveDashboard in development
  if Application.compile_env(:poe_system, :dev_routes) do
    # If you want to use the LiveDashboard in production, you should put
    # it behind authentication and allow only admins to access it.
    # If your application does not have an admins-only section yet,
    # you can use Plug.BasicAuth to set up some basic authentication
    # as long as you are also using SSL (which you should anyway).
    import Phoenix.LiveDashboard.Router

    scope "/dev" do
      pipe_through :browser

      live_dashboard "/dashboard", metrics: PoeSystemWeb.Telemetry
    end
  end
end
