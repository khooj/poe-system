defmodule PoeSystem.Stash do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  schema "stashes" do
    field :id, :string
    field :item_id, :string
  end

  def changeset(stash, attrs \\ %{}) do
    stash
    |> cast(attrs, [:id, :item_id], empty_values: [:id])
    |> validate_required([:id, :item_id])
    |> validate_length(:id, min: 1)
    |> validate_length(:item_id, min: 1)
  end
end
