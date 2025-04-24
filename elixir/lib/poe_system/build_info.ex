defmodule PoeSystem.BuildInfo do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query

  alias PoeSystem.Repo
  alias PoeSystem.BuildInfo.BuildData

  @primary_key false

  schema "builds" do
    field :id, Ecto.UUID, primary_key: true
    field :data, :map
    field :processed, :boolean, default: false

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(build_info, attrs \\ %{}) do
    build_info
    |> cast(attrs, [:id, :processed, :data])
    # |> cast_embed(:data, required: true)
    |> validate_required([:id, :data, :processed])
  end

  def add_build_changeset(id, data) do
    changeset(%__MODULE__{}, %{
      id: id,
      data: data
    })
  end

  def add_build(id, data) do
    add_build_changeset(id, data)
    |> Repo.insert()
  end

  @spec get_build(Ecto.UUID.t()) :: %__MODULE__{}
  def get_build(id) do
    Repo.get!(__MODULE__, id)
  end

  def get_ids() do
    Repo.all(from b in __MODULE__, select: [b.id])
    |> Enum.flat_map(& &1)
  end

  def update_build(build, attrs) do
    changeset(build, attrs)
    |> Repo.update()
  end
end
