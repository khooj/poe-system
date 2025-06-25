defmodule PoeSystem.Items.Item do
  use Ecto.Schema
  import Ecto.Changeset
  alias __MODULE__

  @primary_key false
  schema "items" do
    field :id, :string, primary_key: true
    field :info, :map, source: :data
    field :basetype, :string
    field :category, :string
    field :subcategory, :string
    field :name, :string
    field :price, :map
    field :rarity, :string
  end

  @type t :: %__MODULE__{}
  @type item_type :: :accessory | :gem | :armor | :weapon | :jewel | :flask
  @type quality :: non_neg_integer()
  @type level :: non_neg_integer()

  @spec changeset(Item.t(), map()) :: Ecto.Changeset.t()
  def changeset(item, attrs) do
    item
    |> cast(attrs, [:id, :info, :basetype, :category, :subcategory, :name, :price, :rarity],
      empty_values: []
    )
    |> validate_required([:id, :info, :basetype, :category, :subcategory, :price, :rarity])
    |> unique_constraint(:id)
  end
end
