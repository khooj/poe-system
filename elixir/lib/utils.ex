defmodule Utils do
  def to_string_key_map(v) when is_struct(v), do: to_string_key_map(Map.from_struct(v))

  def to_string_key_map(v) when is_map(v) do
    v
    |> Enum.map(fn
      {k, v} when is_atom(k) -> {Atom.to_string(k), to_string_key_map(v)}
      {k, v} -> {k, v}
    end)
    |> Enum.into(%{})
  end

  def to_string_key_map(v) when is_list(v) do
    v
    |> Enum.map(&to_string_key_map/1)
  end

  def to_string_key_map(v), do: v

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

  def to_atom_key_map(k) when is_map(k) do
    k
    |> Enum.map(fn
      {k, v} when is_binary(k) -> {String.to_existing_atom(k), v}
      {k, v} -> {k, v}
    end)
    |> Enum.into(%{})
  end

  def all_kv do
    fn op, data, next -> all_kv(op, data, next) end
  end

  defp all_kv(:get, data, next) when is_map(data) do
    data |> Enum.into([]) |> next.()
  end

  defp all_kv(:get_and_update, data, next) when is_map(data) do
    case next.(data |> Enum.into([])) do
      {get, update} -> {get, update |> Enum.into(%{})}
      :pop -> raise "in all_kv/0 :pop not implemented"
    end
  end
end
