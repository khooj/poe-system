defmodule PoeSystem.BuildInfoPreview do
  require Protocol
  alias PoeSystem.BuildInfoPreview
  alias PoeSystem.BuildInfo.BuildData
  alias PoeSystem.Repo
  alias Ecto.UUID
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false

  schema "build_info_previews" do
    field :data, :map
    field :id, Ecto.UUID, primary_key: true
    field :itemset, :string
    field :skillset, :string
    field :pob, :string

    timestamps(type: :utc_datetime)
  end

  Protocol.derive(Jason.Encoder, BuildInfoPreview, only: [:id, :itemset, :skillset, :pob, :data])

  @doc false
  def changeset(build_info_preview, attrs) do
    build_info_preview
    |> cast(attrs, [:id, :itemset, :skillset, :pob, :data])
    |> validate_required([:id, :data, :itemset, :skillset, :pob])
  end

  def add_build_changeset(data, itemset, skillset, pob) do
    changeset(%__MODULE__{}, %{
      id: UUID.bingenerate(),
      data: data,
      itemset: itemset,
      skillset: skillset,
      pob: pob
    })
  end

  def add_build(data, itemset, skillset, pob) do
    add_build_changeset(data, itemset, skillset, pob)
    |> Repo.insert()
  end

  def get_build(id) do
    Repo.get!(__MODULE__, id)
  end

  def remove(id) do
    Repo.delete(%__MODULE__{id: id})
  end

  def update(build, config) do
    changeset(build, %{data: config})
    |> Repo.update()
  end
end
