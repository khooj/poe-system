defmodule PoeSystem.Items.Property do
  use Ecto.Schema
  import Ecto.Changeset
  require Protocol

  @primary_key false
  embedded_schema do
    field :name, :string
    field :value, :string
    field :augmented, :boolean
  end

  Protocol.derive(Jason.Encoder, __MODULE__)

  def changeset(struct, data) do
    struct
    |> cast(data, [:name, :value, :augmented])
  end

  def from_json(data) do
    %__MODULE__{}
    |> changeset(data)
    |> apply_changes()
  end
end
