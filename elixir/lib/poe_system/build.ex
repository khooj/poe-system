defmodule PoeSystem.Build do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query
  alias PoeSystem.Repo
  require Protocol
  alias PoeSystem.EctoTypes.Binary
  alias PoeSystem.Build.{ProvidedItems, FoundItems}
  alias __MODULE__

  @primary_key false

  schema "builds" do
    field :id, Ecto.UUID, primary_key: true
    field :provided, Binary
    field :found, Binary
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
    |> cast(attrs, [:id, :processed, :itemset, :skillset, :pob, :fixed, :provided, :found])
    |> validate_required([:id, :provided, :itemset, :skillset, :pob])
  end

  def internal_change(data) do
    %__MODULE__{}
    |> change(data)
  end

  @spec get_build(Ecto.UUID.t()) :: Build.t() | nil
  def get_build(id) do
    Repo.get(__MODULE__, id)
  end

  @spec get_provided(Ecto.UUID.t()) :: map()
  def get_provided(id) do
    Repo.one!(from b in __MODULE__, where: b.id == ^id, select: b.provided)
  end

  @spec get_found(Ecto.UUID.t()) :: map() | nil
  def get_found(id) do
    Repo.one(from b in __MODULE__, where: b.id == ^id, select: b.found)
  end
end
