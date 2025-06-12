defmodule PoeSystem.LatestStash do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false
  schema "latest_stash" do
    field :id, :string, primary_key: true
  end

  def changeset(ls, attrs \\ %{}) do
    ls
    |> cast(attrs, [:id], empty_values: [])
    |> validate_required([:id])
    |> validate_length(:id, min: 1)
  end
end
