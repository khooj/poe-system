defmodule PoeSystemWeb.Poe1ControllerTest do
  use PoeSystemWeb.ConnCase
  alias PoeSystem.Testdata

  test "GET /", %{conn: conn} do
    conn = get(conn, ~p"/poe1")
    assert response(conn, 200)
    assert inertia_component(conn)
  end

  test "POST /extract", %{conn: conn} do
    conn =
      post(conn, ~p"/api/extract", %{
        "itemset" => "Level 13 example",
        "pobData" => Testdata.pobdata_file(),
        "skillset" => "Maps"
      })

    assert %{"config" => _} = json_response(conn, 200)
  end

  test "POST /new", %{conn: conn} do
    cfg = Testdata.extract_config()

    conn =
      post(conn, ~p"/poe1/new",
        config: cfg,
        itemset: "Level 13 example",
        skillset: "Maps",
        pobData: Testdata.pobdata_file()
      )

    assert redirected_to(conn) =~ "/build/"
  end

  describe "GET /build" do
    setup %{conn: conn} do
      cfg = Testdata.extract_config()

      conn =
        post(conn, ~p"/poe1/new",
          config: cfg,
          itemset: "Level 13 example",
          skillset: "Maps",
          pobData: Testdata.pobdata_file()
        )

      %{id: id} = redirected_params(conn)
      %{id: id}
    end

    test "GET /build", %{conn: conn, id: id} do
      conn = get(conn, ~p"/poe1/build/#{id}")

      assert response(conn, 200)
      assert inertia_component(conn)
      assert %{data: _} = inertia_props(conn)
    end
  end
end
