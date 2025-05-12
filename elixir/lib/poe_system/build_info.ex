defmodule PoeSystem.BuildInfo do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query
  alias PoeSystem.Repo
  require Protocol

  @primary_key false

  schema "builds" do
    field :id, Ecto.UUID, primary_key: true
    field :data, :map
    field :processed, :boolean, default: false
    field :itemset, :string
    field :skillset, :string
    field :pob, :string
    field :fixed, :boolean, default: false

    timestamps(type: :utc_datetime)
  end

  Protocol.derive(Jason.Encoder, __MODULE__, except: [:__meta__])

  @doc false
  def changeset(build_info, attrs \\ %{}) do
    build_info
    |> cast(attrs, [:id, :processed, :data, :itemset, :skillset, :pob, :fixed])
    |> validate_required([:id, :data, :itemset, :skillset, :pob])
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
