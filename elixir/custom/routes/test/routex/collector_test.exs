defmodule Routes.CollectorTest do
  use ExUnit.Case

  describe "collect/1" do
    test "collects all routes from a Phoenix router" do
      routes = Routes.Collector.collect(Routes.TestRouter)

      assert length(routes) == 6

      # Test route structure
      route = Enum.find(routes, &(&1.path == "/users/:id"))
      assert route.verb == :get
      assert route.plug == Routes.TestController
      assert route.plug_opts == :show
    end

    test "includes path parameters in collected routes" do
      routes = Routes.Collector.collect(Routes.TestRouter)
      route = Enum.find(routes, &(&1.path == "/users/:id"))

      assert route.path =~ ":id"
    end
  end
end
