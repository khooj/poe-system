defmodule PoeSystem.EctoTypes.Price do
  use Ecto.Type
  def type, do: :map

  def cast({:custom, b, c} = info) when is_binary(b) and is_integer(c) do
    {:ok, info}
  end

  def cast({a, b} = info) when is_atom(a) and is_integer(b) do
    {:ok, info}
  end

  def cast(_), do: :error

  def load(%{"Custom" => value}) do
    [t, v] = value

    {:ok, {:custom, t, v}}
  end

  def load(%{"Chaos" => value}), do: {:ok, {:chaos, value}}
  def load(%{"Divine" => value}), do: {:ok, {:divine, value}}

  def dump({:chaos, b}) when is_integer(b), do: {:ok, %{"Chaos" => b}}
  def dump({:divine, b}) when is_integer(b), do: {:ok, %{"Divine" => b}}
  def dump({:custom, b, c}) when is_binary(b) and is_integer(c) do
    {:ok, %{"Custom" => [b, c]}}
  end

  def dump(_), do: :error
end
