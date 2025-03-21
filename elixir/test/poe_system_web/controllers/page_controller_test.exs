defmodule PoeSystemWeb.PageControllerTest do
  alias PoeSystem.BuildInfo
  alias PoeSystem.Repo
  use PoeSystemWeb.ConnCase

  defp pobdata_file() do
    String.trim(File.read!(Path.join([__DIR__, "../../testdata/pob.txt"])))
  end

  defp extract_config() do
    {:ok, data} = RustPoe.Native.extract_build_config(pobdata_file(), "Level 13 example", "Maps")
    data
  end

  test "GET /", %{conn: conn} do
    conn = get(conn, ~p"/")
    assert response(conn, 200)
    assert inertia_component(conn)
  end

  test "POST /new", %{conn: conn} do
    cfg = extract_config()

    conn =
      post(conn, ~p"/new", %{
        "itemset" => "Level 13 example",
        "pobData" => pobdata_file(),
        "skillset" => "Maps",
        "userConfig" => cfg
      })

    assert %{id: id} = redirected_params(conn)
    assert redirected_to(conn) =~ "/build/#{id}"
  end

  describe "GET /build" do
    setup %{conn: conn} do
      cfg = extract_config()

      conn =
        post(conn, ~p"/new", %{
          "itemset" => "Level 13 example",
          "pobData" => pobdata_file(),
          "skillset" => "Maps",
          "userConfig" => cfg
        })

      %{id: id} = redirected_params(conn)
      {:ok, id: id}
    end

    test "retrieve build", %{conn: conn, id: id} do
      data = Repo.one!(BuildInfo)
      conn = get(conn, ~p"/build/#{id}")

      assert response(conn, 200)
      assert inertia_component(conn)
      assert %{data: respData} = inertia_props(conn)
      assert data.data == respData
    end
  end
end
