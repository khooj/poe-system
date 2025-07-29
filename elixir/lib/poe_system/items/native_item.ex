defmodule PoeSystem.Items.NativeItem do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.{ItemConfig, Item}
  alias __MODULE__
  alias Utils

  @type t :: %__MODULE__{}

  @primary_key false
  embedded_schema do
    embeds_one :config, ItemConfig
    embeds_one :item, Item
  end

  def changeset(struct, data \\ %{}) do
    struct
    |> cast(data, [])
    |> cast_embed(:config)
    |> cast_embed(:item)
  end

  def from_json(data) when is_map(data) do
    changeset(%__MODULE__{}, data)
    |> apply_changes()
  end
end
