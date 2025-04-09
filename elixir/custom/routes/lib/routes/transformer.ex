defmodule Routes.Transformer do
  @moduledoc """
  Transforms Phoenix route maps into simplified route structures.

  Provides functionality to convert complex Phoenix route maps into a more
  normalized format that is easier to work with.
  """
  @type route_map :: %{
          path: String.t(),
          plug_opts: atom(),
          verb: atom(),
          plug: module()
        }

  @type transformed_route :: %{
          name: String.t(),
          action: atom(),
          path: String.t(),
          method: String.t(),
          controller: String.t(),
          params: [String.t()]
        }

  @doc """
  Transforms a Phoenix route map into a simplified route structure.

  Takes a route map containing the path, plug options, HTTP verb and plug module,
  and converts it into a transformed route with normalized fields.

  Returns `nil` if the input is invalid or cannot be transformed.
  """
  @spec transform(route_map()) :: transformed_route() | nil
  def transform(%{path: path, plug_opts: plug_opts, verb: verb, plug: plug})
      when is_binary(path) and not is_nil(plug_opts) do
    method =
      verb
      |> Atom.to_string()
      |> String.upcase()

    name = get_name(path, plug_opts)

    %{
      name: name,
      action: plug_opts,
      path: path,
      method: method,
      controller: inspect(plug),
      params: extract_params(path)
    }
  end

  def transform(_), do: nil

  @spec get_name(String.t(), atom()) :: String.t()
  defp get_name(path, plug_opts) do
    case path do
      "/" ->
        "index"

      _ ->
        path =
          path
          |> String.trim_leading("/")
          |> String.split("/")
          |> Enum.reject(&String.starts_with?(&1, ":"))
          |> Enum.join(".")

        if plug_opts == [] do
          path
        else
          "#{path}.#{plug_opts}"
        end
    end
  end

  @spec extract_params(String.t()) :: [String.t()]
  defp extract_params(path) when is_binary(path) do
    Regex.scan(~r/:([^\/]+)/, path)
    |> Enum.map(fn [_, param] -> param end)
  end

  defp extract_params(_), do: []
end
