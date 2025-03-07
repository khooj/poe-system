defmodule PoeSystem.BuildInfo do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false

  schema "builds" do
    field :id, Ecto.UUID, primary_key: true

    embeds_one :data, BuildData, primary_key: false do
      field :helmet, :string
    end

    field :processed, :boolean, default: false

    timestamps(type: :utc_datetime, inserted_at: :created_at)
  end

  @doc false
  def changeset(build_info, attrs) do
    build_info
    |> cast(attrs, [:id, :processed])
    |> cast_embed(:data, required: true, with: &data_changeset/2)
    |> validate_required([:id, :data, :processed])
  end

  def data_changeset(data, attrs \\ %{}) do
    data
    |> cast(attrs, [:helmet])
  end
end
