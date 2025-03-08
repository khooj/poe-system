defmodule PoeSystem.BuildInfo.FoundBuildItems do
  use Ecto.Schema

  embedded_schema do
    field :helmet, :map
    field :body, :map
    field :boots, :map
    field :gloves, :map
    field :weapon1, :map
    field :weapon2, :map
    field :ring1, :map
    field :ring2, :map
    field :belt, :map
    field :flasks, {:array, :map}
    field :gems, {:array, :map}
    field :jewels, {:array, :map}
  end
end
