defmodule PoeSystemWeb.PageController do
  alias PoeSystem.BuildInfo
  use PoeSystemWeb, :controller

  def index(conn, _params) do
    build_ids = BuildInfo.get_ids()

    conn
    |> assign_prop(:build_ids, build_ids)
    |> render_inertia("Index")
  end

  def new(conn, %{
        "itemset" => itemset,
        "pobData" => pob_data,
        "skillset" => skillset,
        "userConfig" => user_config
      }) do
    {:ok, extracted_config} = RustPoe.Native.extract_build_config(pob_data, itemset, skillset)

    {:ok, validated_config} =
      RustPoe.Native.validate_and_apply_config(extracted_config, user_config)

    {:ok, data} = BuildInfo.add_build(validated_config)

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
