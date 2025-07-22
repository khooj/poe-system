defmodule PoeSystem.Items.ItemInfo do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.ItemMod

  @primary_key false
  embedded_schema do
    embeds_many :mods, ItemMod
  end

  def changeset(struct, data) do
    struct
    |> cast(data, [])
    |> cast_embed(:mods)
  end
end
