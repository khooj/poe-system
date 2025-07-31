defmodule PoeSystem.Build.ProvidedItems do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.NativeItem

  @type t :: %__MODULE__{}

  @primary_key false
  embedded_schema do
    embeds_one :amulet, NativeItem
    embeds_one :helmet, NativeItem
    embeds_one :body, NativeItem
    embeds_one :boots, NativeItem
    embeds_one :gloves, NativeItem
    embeds_one :weapon1, NativeItem
    embeds_one :weapon2, NativeItem
    embeds_one :ring1, NativeItem
    embeds_one :ring2, NativeItem
    embeds_one :belt, NativeItem
    embeds_many :flasks, NativeItem
    embeds_many :gems, NativeItem
    embeds_many :jewels, NativeItem
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
