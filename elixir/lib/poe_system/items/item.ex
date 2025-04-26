defmodule PoeSystem.Items.Item do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  schema "items" do
    field :id, :string, primary_key: true
    field :info, :map, source: :data
    field :basetype, :string
    field :category, :string
    field :subcategory, :string
    field :name, :string
    field :price, :map
  end

  def changeset(item, attrs) do
    item
    |> cast(attrs, [:id, :info, :basetype, :category, :subcategory, :name, :price],
      empty_values: []
    )
    |> validate_required([:id, :info, :basetype, :category, :subcategory, :price])
  end
end
