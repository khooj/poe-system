defmodule PoeSystem.Items.NativeItem do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Items.{ItemConfig, ItemStruct}

  @primary_key false
  embedded_schema do
    embeds_one :config, ItemConfig
    embeds_one :item, ItemStruct
  end

  def from_json(data) when is_map(data) do
    %__MODULE__{}
      |> cast(data, [])
      |> cast_embed(:config)
      |> cast_embed(:item)
      |> apply_changes()
  end
end
