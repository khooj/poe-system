defmodule PoeSystem.Items.Item do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  schema "items" do
    field :id, :string, primary_key: true
    field :data, :map
    field :basetype, :string
    field :category, :string
    field :subcategory, :string
    field :name, :string
    field :price, :map
  end

  def changeset(item, attrs) do
    item
    |> cast(attrs, [:id, :data, :basetype, :category, :subcategory, :name, :price])
    |> validate_required([:id, :data, :basetype, :category, :subcategory, :name, :price])
  end
end
