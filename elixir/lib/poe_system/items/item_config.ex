defmodule PoeSystem.Items.ItemConfig do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  embedded_schema do
    field :basetype, :boolean
    field :option, :map
  end

  def changeset(struct, data) do
    struct
    |> cast(data, [:basetype, :option])
  end
end
