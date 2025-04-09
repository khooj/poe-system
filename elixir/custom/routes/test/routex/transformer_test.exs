defmodule Routes.TransformerTest do
  use ExUnit.Case

  defp build_test_route(path, action) do
    %{
      path: path,
      plug_opts: action,
      verb: :get,
      plug: Routes.TestController
    }
  end

  describe "transform/1" do
    test "transforms a Phoenix route into Routes format" do
      phoenix_route = build_test_route("/users/:id", :show)

      transformed =
        Routes.Transformer.transform(phoenix_route)

      assert transformed == %{
               name: "users.show",
               path: "/users/:id",
               method: "GET",
               controller: "Routes.TestController",
               action: :show,
               params: ["id"]
             }
    end

    test "handles routes without parameters" do
      phoenix_route = build_test_route("/users", :list)

      transformed = Routes.Transformer.transform(phoenix_route)

      assert transformed == %{
               name: "users.list",
               path: "/users",
               method: "GET",
               controller: "Routes.TestController",
               action: :list,
               params: []
             }
    end

    test "returns error for invalid route" do
      phoenix_route = %{invalid: "route"}

      transformed = Routes.Transformer.transform(phoenix_route)

      assert is_nil(transformed)
    end
  end
end
