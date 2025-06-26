defmodule PoeSystem.Items.Item do
  use Ecto.Schema
  import Ecto.Changeset
  require Protocol
  alias __MODULE__

  schema "items" do
    field :item_id, :string
    field :info, :map
    field :basetype, :string
    field :category, :string
    field :subcategory, :string
    field :name, :string
    field :price, :map
    field :rarity, :string
  end

  Protocol.derive(Jason.Encoder, __MODULE__, except: [:__meta__])

  @type t :: %__MODULE__{}
  @type item_type :: :accessory | :gem | :armor | :weapon | :jewel | :flask
  @type quality :: non_neg_integer()
  @type level :: non_neg_integer()

  @spec changeset(Item.t(), map()) :: Ecto.Changeset.t()
  def changeset(item, attrs \\ %{}) do
    item
    |> cast(
      attrs,
      [:id, :item_id, :info, :basetype, :category, :subcategory, :name, :price, :rarity],
      empty_values: []
    )
    |> validate_required([
      :item_id,
      :info,
      :basetype,
      :category,
      :subcategory,
      :price,
      :rarity
    ])
    |> unique_constraint(:item_id)
  end
end
