defmodule PoeSystem.Items.Mod do
  use Ecto.Schema
  import Ecto.Changeset
  require Protocol

  @primary_key false
  embedded_schema do
    field :stat_id, :string
    field :text, :string
  end

  Protocol.derive(Jason.Encoder, __MODULE__)

  def changeset(struct, data) do
    struct
    |> cast(data, [:stat_id, :text])
  end

  def from_json(data) do
    %__MODULE__{}
    |> changeset(data)
    |> apply_changes()
  end
end
