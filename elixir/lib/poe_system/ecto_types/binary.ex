defmodule PoeSystem.EctoTypes.Binary do
  use Ecto.Type
  def type, do: :binary

  def cast(data) do
    {:ok, data}
  end

  def load(data) when is_binary(data) do
    data = data
      |> :erlang.binary_to_term(data)
    {:ok, data}
  end

  def dump(data) do
    {:ok, :erlang.term_to_binary(data)}
  end
end
