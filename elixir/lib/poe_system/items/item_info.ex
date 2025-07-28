defmodule PoeSystem.Items.ItemInfo do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.ItemMod

  @primary_key false
  embedded_schema do
    embeds_many :mods, ItemMod
    field :quality, :integer
    field :type, :string
  end

  def changeset(struct, data) do
    struct
    |> cast(data, [:quality, :type])
    |> cast_embed(:mods)
  end

  def from_json(data) when is_map(data) do
    changeset(%__MODULE__{}, data)
    |> apply_changes()
  end
end
