defmodule PoeSystemWeb.Router do
  use PoeSystemWeb, :router
  use Routes

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    plug :put_root_layout, html: {PoeSystemWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers
    plug Inertia.Plug
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", PoeSystemWeb do
    pipe_through :browser

    get "/", IndexController, :index
  end

  scope "/poe1", PoeSystemWeb do
    pipe_through :browser

    get "/", Poe1Controller, :index
    post "/new/:id", Poe1Controller, :new
    post "/extract", Poe1Controller, :extract
    get "/preview/:id", Poe1Controller, :preview
    patch "/preview/:id", Poe1Controller, :update_preview
    get "/build/:id", Poe1Controller, :get_build
  end

  # Other scopes may use custom stacks.
  # scope "/api", PoeSystemWeb do
  #   pipe_through :api
  # end

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
