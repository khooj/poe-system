defmodule Utils do
  def to_string_key_map(%{} = v) when not is_struct(v) do
    v
    |> Enum.map(fn
      {k, v} when is_atom(k) -> {Atom.to_string(k), v}
      {k, v} -> {k, v}
    end)
    |> Enum.into(%{})
  end

  def to_string_key_map(v) when is_struct(v), do: to_string_key_map(Map.from_struct(v))

  # sobelow_skip ["DOS.StringToAtom"]
  # SECURITY: String.to_atom
  def unsafe_to_atom_key_map(k) when is_map(k) do
    k
    |> Enum.map(fn
      {k, v} when is_binary(k) -> {String.to_atom(k), v}
      {k, v} -> {k, v}
    end)
    |> Enum.into(%{})
  end
end
