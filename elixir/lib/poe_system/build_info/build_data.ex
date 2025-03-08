defmodule PoeSystem.BuildInfo.BuildData do
  use Ecto.Schema
  import Ecto.Changeset

  embedded_schema do
    field :provided, :map
    field :found, :map
  end

  def changeset(data, attrs \\ %{}) do
    data
    |> cast(attrs, [:provided, :found])
  end
end
