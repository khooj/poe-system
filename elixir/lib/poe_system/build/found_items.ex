defmodule PoeSystem.Build.FoundItems do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.Item

  @type t :: %__MODULE__{}

  @primary_key false
  embedded_schema do
    embeds_one :amulet, Item
    embeds_one :helmet, Item
    embeds_one :body, Item
    embeds_one :boots, Item
    embeds_one :gloves, Item
    embeds_one :weapon1, Item
    embeds_one :weapon2, Item
    embeds_one :ring1, Item
    embeds_one :ring2, Item
    embeds_one :belt, Item
    embeds_many :flasks, Item
    embeds_many :gems, Item
    embeds_many :jewels, Item
  end

  def changeset(struct, data \\ %{}) do
    struct
    |> cast(data, [])
    |> cast_embed(:amulet)
    |> cast_embed(:helmet)
    |> cast_embed(:body)
    |> cast_embed(:boots)
    |> cast_embed(:gloves)
    |> cast_embed(:weapon1)
    |> cast_embed(:weapon2)
    |> cast_embed(:ring1)
    |> cast_embed(:ring2)
    |> cast_embed(:belt)
    |> cast_embed(:flasks)
    |> cast_embed(:gems)
    |> cast_embed(:jewels)
  end

  def from_json(data) when is_map(data) do
    changeset(%__MODULE__{}, data)
    |> apply_changes()
  end
end
