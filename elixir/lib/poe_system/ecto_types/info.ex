defmodule PoeSystem.EctoTypes.Info do
  use Ecto.Type
  alias PoeSystem.Items.{Mod, Property}
  def type, do: :map

  def cast({a, b} = info) when is_atom(a) and is_map(b) do
    {:ok, info}
  end

  def cast(_), do: :error

  def load(%{"type" => type} = info) do
    t =
      type
      |> String.downcase()
      |> String.to_atom()

    values =
      info
      |> Enum.filter(fn {k, _} -> k != "type" end)
      |> Enum.map(fn
        {"mods", v} -> {:mods, Enum.map(v, &Mod.from_json/1)}
        {"properties", v} -> {:properties, Enum.map(v, &Property.from_json/1)}
        {k, v} when is_binary(k) -> {String.to_atom(k), v}
      end)
      |> Enum.into(%{})

    {:ok, {t, values}}
  end

  def dump({a, b}) when is_atom(a) and is_map(b) do
    t =
      a
      |> Atom.to_string()
      |> String.capitalize()

    v =
      (Enum.into(b, []) ++ [{"type", t}])
      |> Enum.into(%{})

    {:ok, v}
  end

  def dump(_), do: :error
end
