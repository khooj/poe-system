defmodule Routes.Processor do
  @moduledoc """
  Processes routes from a module and generates the corresponding JavaScript/TypeScript files.
  """

  require Logger
  @default_js_path "assets/js"

  @doc """
  Processes routes from the given module and generates JavaScript/TypeScript files.

  Returns `{:ok, routes}` on success or `{:error, reason}` on failure.

  ## Examples

      iex> Routes.Processor.process_routes(MyAppWeb.Router)
      {:ok, [%{path: "/", name: "root"}, %{path: "/users", name: "users"}]}

  """
  def process_routes(module) do
    with {:ok, routes} <- collect_and_transform_routes(module),
         :ok <- ensure_output_directory(),
         :ok <- generate_files(routes) do
      {:ok, routes}
    else
      error -> error
    end
  end

  defp collect_and_transform_routes(module) do
    routes =
      module
      |> Routes.Collector.collect()
      |> Enum.map(&Routes.Transformer.transform/1)
      |> Enum.reject(&is_nil/1)

    {:ok, routes}
  end

  defp ensure_output_directory do
    output_path()
    |> File.mkdir_p!()

    :ok
  end

  defp generate_files(routes) do
    if Application.get_env(:routes, :typescript, false) do
      Routes.Generator.write_javascript_with_types(routes, output_path())
    else
      Routes.Generator.write_javascript(routes, output_path())
    end

    :ok
  end

  defp output_path, do: Application.get_env(:routes, :routes_path, @default_js_path)
end
