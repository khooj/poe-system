defmodule PoeSystem.BuildInfo.ItemWithConfig do
  use Ecto.Schema

  embedded_schema do
    field :config, {:array, :map}
    field :item, :map
  end
end
