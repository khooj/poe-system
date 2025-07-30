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
  end

  pipeline :api do
    plug :accepts, ["json"]
    plug PoeSystemWeb.Plug.RateLimiter
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
      live "/", Poe1BuildCalcIndexLive, :new
      live "/preview", Poe1BuildCalcIndexLive, :preview
      live "/:id", Poe1BuildCalcBuildLive
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
