defmodule PoeSystemWeb.Poe1ControllerTest do
  use PoeSystemWeb.ConnCase
  alias PoeSystem.Testdata

  setup do
    {itemset, skillset} = Testdata.config_info()

    %{
      itemset: itemset,
      skillset: skillset
    }
  end

  test "GET /", %{conn: conn} do
    conn = get(conn, ~p"/poe1")
    assert response(conn, 200)
    assert inertia_component(conn)
  end

  test "POST /extract", %{conn: conn, itemset: itemset, skillset: skillset} do
    conn =
      post(conn, ~p"/api/extract", %{
        "itemset" => itemset,
        "pobData" => Testdata.pobdata_file(),
        "skillset" => skillset
      })

    assert %{"config" => _} = json_response(conn, 200)
  end

  test "POST /new", %{conn: conn, itemset: itemset, skillset: skillset} do
    cfg = Testdata.extract_config()

    conn =
      post(conn, ~p"/poe1/new",
        config: cfg,
        itemset: itemset,
        skillset: skillset,
        pobData: Testdata.pobdata_file()
      )

    assert redirected_to(conn) =~ "/build/"
  end

  describe "GET /build" do
    setup %{conn: conn, itemset: itemset, skillset: skillset} do
      cfg = Testdata.extract_config()

      conn =
        post(conn, ~p"/poe1/new",
          config: cfg,
          itemset: itemset,
          skillset: skillset,
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
