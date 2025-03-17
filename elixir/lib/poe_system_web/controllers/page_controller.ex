defmodule PoeSystemWeb.PageController do
  alias PoeSystem.BuildInfo
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    build_ids = BuildInfo.get_ids()
    IO.inspect(build_ids)

    conn
    |> assign_prop(:build_ids, build_ids)
    |> render_inertia("Index")
  end

  def new(conn, %{"itemset" => itemset, "pobData" => pobData, "skillset" => skillset}) do
    {:ok, data} = RustPoe.Native.extract_build_config(pobData, itemset, skillset)
    {:ok, data} = BuildInfo.add_build(data)

    conn
    |> redirect(to: ~p"/new_build/#{data.id}")
  end

  def get_build(conn, %{"id" => id}) do
    data = BuildInfo.get_build(id)

    conn
    |> assign_prop(:data, data.data)
    |> render_inertia("Build")
  end

  def new_build(conn, %{"id" => id}) do
    data = BuildInfo.get_build(id)

    conn
    |> assign_prop(:data, data.data.provided)
    |> render_inertia("NewBuild")
  end
end
