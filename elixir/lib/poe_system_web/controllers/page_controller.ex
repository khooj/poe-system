defmodule PoeSystemWeb.PageController do
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    conn
      |> assign_prop(:text, "Hello world from phoenix")
      |> render_inertia("Index")
  end
end
