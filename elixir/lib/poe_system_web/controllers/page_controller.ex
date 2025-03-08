defmodule PoeSystemWeb.PageController do
  alias PoeSystem.BuildInfo
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    conn
    |> assign_prop(:text, "Hello world from phoenix")
    |> render_inertia("Index")
  end

  def new(conn, params) do
    IO.inspect(params)

    conn
    |> redirect(to: ~p"/")
  end

  def get_build(conn, %{"id" => id}) do
    data = BuildInfo.get_build(id)

    conn
    |> assign_prop(:data, data.data)
    |> render_inertia("Build")
  end
end
