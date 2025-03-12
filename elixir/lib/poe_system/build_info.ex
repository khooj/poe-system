defmodule PoeSystem.BuildInfo do
  use Ecto.Schema
  import Ecto.Query
  import Ecto.Changeset

  alias Ecto.UUID
  alias PoeSystem.Repo
  alias PoeSystem.BuildInfo.BuildData

  @primary_key false

  schema "builds" do
    field :id, Ecto.UUID, primary_key: true
    embeds_one :data, BuildData
    field :processed, :boolean, default: false

    timestamps(type: :utc_datetime, inserted_at: :created_at)
  end

  @doc false
  def changeset(build_info, attrs) do
    build_info
    |> cast(attrs, [:id, :processed])
    |> cast_embed(:data, required: true)
    |> validate_required([:id, :data, :processed])
  end

  def add_build(data) do
    changeset(%__MODULE__{}, %{
      id: UUID.bingenerate(),
      data: data
    })
    |> Repo.insert()
  end

  def get_build(id) do
    Repo.get!(__MODULE__, id)
  end
end
