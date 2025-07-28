defmodule PoeSystem.Items.Item do
  use Ecto.Schema
  import Ecto.Changeset
  require Protocol
  alias __MODULE__
  alias PoeSystem.Items.ItemInfo

  @primary_key false
  schema "items" do
    field :item_id, :id, primary_key: true
    field :id, :string
    field :info, PoeSystem.EctoTypes.Info
    field :basetype, :string
    field :category, Ecto.Enum, values: [
      :weapons, 
      :gems, 
      :jewels, 
      :accessories, 
      :flasks, 
      :armour,
    ]
    field :subcategory, Ecto.Enum, values: [
      :weapon, 
      :gem, 
      :jewel, 
      :amulet, 
      :utility_flask, 
      :life_flask, 
      :mana_flask, 
      :hybrid_flask,
      :boots,
      :body_armour,
      :shield,
      :gloves,
      :helmets,
      :belt,
      :ring,
      :quiver,
    ]
    field :name, :string
    field :price, PoeSystem.EctoTypes.Price
    field :rarity, :string
  end

  Protocol.derive(Jason.Encoder, __MODULE__, except: [:__meta__])

  @type t :: %__MODULE__{}
  @type item_type :: :accessory | :gem | :armor | :weapon | :jewel | :flask
  @type quality :: non_neg_integer()
  @type level :: non_neg_integer()
  @type item_with_config :: map()

  @spec changeset(Item.t(), map()) :: Ecto.Changeset.t()
  def changeset(item, attrs \\ %{}) do
    item
    |> cast(
      attrs,
      [:id, :item_id, :basetype, :category, :subcategory, :name, :price, :rarity, :info],
      empty_values: []
    )
    |> validate_required([
      :id,
      :info,
      :basetype,
      :category,
      :subcategory,
      :price,
      :rarity
    ])
  end

  @spec internal_change(Item.t(), map()) :: Ecto.Changeset.t()
  def internal_change(item, attrs \\ %{}) do
    item
    |> change(attrs)
    |> validate_required([
      :id,
      :info,
      :basetype,
      :category,
      :subcategory,
      :price,
      :rarity
    ])
  end

  def from_json(data) when is_map(data) do
    changeset(%__MODULE__{}, data)
    |> apply_changes()
  end
end
