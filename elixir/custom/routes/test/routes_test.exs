defmodule RoutesTest do
  use ExUnit.Case
  doctest Routes

  @output_dir "test/support"
  @js_path Path.join(@output_dir, "routes.js")
  @dts_path Path.join(@output_dir, "routes.d.ts")

  setup_all do
    Application.put_env(:routes, :routes_path, @output_dir)
    Application.put_env(:routes, :typescript, true)
    File.mkdir_p!(@output_dir)

    Code.compile_file("test/support/test_router.ex")

    on_exit(fn ->
      File.rm(@js_path)
      File.rm(@dts_path)
      Application.delete_env(:routes, :routes_path)
    end)

    {:ok, js_path: @js_path, dts_path: @dts_path}
  end

  describe "JavaScript route file generation" do
    setup %{js_path: js_path} do
      assert File.exists?(js_path), "JavaScript route file was not generated"
      {:ok, content: File.read!(js_path)}
    end

    test "includes required utility functions", %{content: content} do
      required_functions = [
        "replaceParams",
        "route",
        "path",
        "method"
      ]

      Enum.each(required_functions, fn func ->
        assert content =~ "function #{func}", "Missing required function: #{func}"
      end)
    end

    test "exports module correctly", %{content: content} do
      assert content =~ "module.exports = Routes", "Missing module.exports"
      assert content =~ "window.Routes = Routes", "Missing window binding"
    end

    test "contains all expected routes", %{content: content} do
      expected_routes = [
        %{path: "/users/:id", methods: ["GET", "PUT", "DELETE"]},
        %{path: "/users", methods: ["GET", "POST"]},
        %{path: "/", methods: ["GET"]}
      ]

      Enum.each(expected_routes, fn route ->
        assert content =~ ~s|"#{route.path}"|, "Missing route path: #{route.path}"

        Enum.each(route.methods, fn method ->
          assert content =~ ~s|"#{method}"|,
                 "Missing HTTP method #{method} for path #{route.path}"
        end)
      end)
    end

    test "defines all controller actions", %{content: content} do
      expected_actions = ["index", "list", "show", "create", "update", "delete"]

      Enum.each(expected_actions, fn action ->
        assert content =~ ~s|"action":"#{action}"|, "Missing controller action: #{action}"
      end)
    end
  end

  describe "TypeScript declaration file generation" do
    setup %{dts_path: dts_path} do
      assert File.exists?(dts_path), "TypeScript declaration file was not generated"
      {:ok, content: File.read!(dts_path)}
    end

    test "includes required TypeScript definitions", %{content: content} do
      required_definitions = [
        "interface Route",
        "declare const Routes:",
        "export as namespace Routes",
        "export = Routes"
      ]

      Enum.each(required_definitions, fn definition ->
        assert content =~ definition, "Missing TypeScript definition: #{definition}"
      end)
    end
  end

  describe "route helper function generation" do
    test "correctly implements parameter replacement", %{js_path: js_path} do
      content = File.read!(js_path)

      required_implementations = [
        %{feature: "parameter replacement", code: "replaceParams"},
        %{feature: "parameter iteration", code: "Object.keys(routeParams).forEach"},
        %{feature: "route lookup", code: "routes.find(r => r.name === name)"}
      ]

      Enum.each(required_implementations, fn %{feature: feature, code: code} ->
        assert content =~ code, "Missing #{feature} implementation: #{code}"
      end)
    end
  end
end
