defmodule PoeSystemWeb.PageController do
  alias PoeSystem.BuildInfo
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    conn
    |> assign_prop(:text, "Hello world from phoenix")
    |> render_inertia("Index")
  end

  def new(conn, %{"itemset" => itemset, "pobData" => pobData}) do
    {:ok, data} = RustPoe.Native.extract_build_config(pobData, itemset)
    {:ok, data} = BuildInfo.add_build(data)

    conn
    |> redirect(to: ~p"/build/#{data.id}")
  end

  def get_build(conn, %{"id" => id}) do
    data = BuildInfo.get_build(id)

    conn
    |> assign_prop(:data, data.data)
    |> render_inertia("Build")
  end
end
