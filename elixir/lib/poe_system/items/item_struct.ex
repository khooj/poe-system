defmodule PoeSystem.Items.ItemStruct do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.ItemInfo

  @primary_key false
  embedded_schema do
    field :basetype, :string
    field :category, :string
    field :id, :string
    embeds_one :info, ItemInfo
    field :name, :string
    field :price, :map
    field :rarity, :string
    field :subcategory, :string
  end

  def changeset(struct, data) do
    struct
    |> cast(data, [:basetype, :category, :id, :name, :price, :rarity, :subcategory])
    |> cast_embed(:info)
  end
end
