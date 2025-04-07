defmodule PoeSystemWeb.Poe1ControllerTest do
  alias PoeSystem.BuildInfo
  alias PoeSystem.Repo
  use PoeSystemWeb.ConnCase
  alias PoeSystem.Testdata

  setup %{conn: conn} do
    conn =
      post(conn, ~p"/poe1/extract", %{
        "itemset" => "Level 13 example",
        "pobData" => Testdata.pobdata_file(),
        "skillset" => "Maps"
      })

    %{id: id} = redirected_params(conn)
    {:ok, id: id}
  end

  test "GET /", %{conn: conn} do
    conn = get(conn, ~p"/poe1")
    assert response(conn, 200)
    assert inertia_component(conn)
  end

  test "POST /extract", %{conn: conn} do
    conn =
      post(conn, ~p"/poe1/extract", %{
        "itemset" => "Level 13 example",
        "pobData" => Testdata.pobdata_file(),
        "skillset" => "Maps"
      })

    assert %{id: id} = redirected_params(conn)
    assert redirected_to(conn) =~ "/preview/#{id}"
  end

  describe "/preview" do
    test "GET /preview", %{conn: conn, id: id} do
      conn = get(conn, ~p"/poe1/preview/#{id}")

      assert inertia_component(conn)
    end

    test "PATCH /preview", %{conn: conn, id: id} do
      cfg = Testdata.extract_config()
      conn = patch(conn, ~p"/poe1/preview", config: cfg, id: id)
      assert response(conn, 302)
      assert %{id: ^id} = redirected_params(conn)
    end
  end

  test "POST /new", %{conn: conn, id: id} do
    conn = post(conn, ~p"/poe1/new/#{id}")
    assert redirected_to(conn) =~ "/build/#{id}"
  end

  test "GET /build", %{conn: conn, id: id} do
    conn = post(conn, ~p"/poe1/new/#{id}")
    conn = get(conn, ~p"/poe1/build/#{id}")

    assert response(conn, 200)
    assert inertia_component(conn)
    assert %{data: respData} = inertia_props(conn)
  end
end
