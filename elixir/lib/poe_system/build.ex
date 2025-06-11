defmodule PoeSystem.Build do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query
  alias PoeSystem.Repo
  require Protocol
  alias PoeSystem.Build.BuildInfo
  alias __MODULE__

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

  @type t :: %__MODULE__{}

  @doc false
  @spec changeset(Build.t(), map()) :: Ecto.Changeset.t()
  def changeset(build_info, attrs \\ %{}) do
    build_info
    |> cast(attrs, [:id, :processed, :data, :itemset, :skillset, :pob, :fixed])
    |> validate_required([:id, :data, :itemset, :skillset, :pob])
  end

  @spec add_build_changeset(String.t(), BuildInfo.t()) :: Ecto.Changeset.t()
  def add_build_changeset(id, data) do
    changeset(%__MODULE__{}, %{
      id: id,
      data: data
    })
  end

  @spec add_build(String.t(), BuildInfo.t()) ::
          {:ok, Build.t()} | {:error, Ecto.Changeset.t()}
  def add_build(id, data) do
    add_build_changeset(id, data)
    |> Repo.insert()
  end

  @spec get_build(Ecto.UUID.t()) :: Build.t() | nil
  def get_build(id) do
    Repo.get(__MODULE__, id)
  end

  @spec get_ids() :: [String.t()]
  def get_ids() do
    Repo.all(from b in __MODULE__, select: [b.id])
    |> Enum.flat_map(& &1)
  end

  @spec update_build(Build.t(), map()) :: {:ok, Build.t()} | {:error, Ecto.Changeset.t()}
  def update_build(build, attrs) do
    changeset(build, attrs)
    |> Repo.update()
  end
end
