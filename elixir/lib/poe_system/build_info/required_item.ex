defmodule PoeSystem.BuildInfo.RequiredItem do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  embedded_schema do
    field :id, :string
    field :basetype, :string
    field :category, :string
    field :subcategory, :string
    field :name, :string
    field :search_basetype, :boolean
    field :search_subcategory, :boolean
    field :info, :map
  end

  def changeset(req_item, attrs \\ %{}) do
    req_item
    |> cast(attrs, [
      :id,
      :basetype,
      :category,
      :subcategory,
      :name,
      :search_basetype,
      :search_subcategory,
      :info
    ])
    |> validate_required([
      :id,
      :basetype,
      :category,
      :subcategory,
      :name,
      :search_basetype,
      :search_subcategory,
      :info
    ])
  end
end
