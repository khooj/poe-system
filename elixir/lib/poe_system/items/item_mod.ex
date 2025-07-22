defmodule PoeSystem.Items.ItemMod do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  embedded_schema do
    field :stat_id, :string
    field :text, :string
  end

  def changeset(struct, data) do
    struct
    |> cast(data, [:stat_id, :text])
  end
end
