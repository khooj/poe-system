defmodule PoeSystemWeb.PageControllerTest do
  alias PoeSystem.BuildInfo
  alias PoeSystem.Repo
  use PoeSystemWeb.ConnCase

  defp pobdata_file() do
    String.trim(File.read!(Path.join([__DIR__, "../../testdata/pob.txt"])))
  end

  test "GET /", %{conn: conn} do
    conn = get(conn, ~p"/")
    assert response(conn, 200)
    assert inertia_component(conn)
  end

  test "POST /new", %{conn: conn} do
    conn =
      post(conn, ~p"/new", %{
        "itemset" => "Level 13 example",
        "pobData" => pobdata_file()
      })

    assert String.contains?(redirected_to(conn), "/build")
  end

  test "GET /build", %{conn: conn} do
    id = Repo.one!(BuildInfo)
    conn = get(conn, ~p"/build/#{id}")

    assert response(conn, 200)
    assert inertia_component(conn)
  end
end
