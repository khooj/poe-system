defmodule PoeSystem.BuildInfo.BuildData do
  alias PoeSystem.BuildInfo.FoundBuildItems
  alias PoeSystem.BuildInfo.BuildItemsWithConfig
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key false

  @derive Jason.Encoder
  embedded_schema do
    embeds_one :provided, :map
    embeds_one :found, :map
  end

  # def changeset(data, attrs \\ %{}) do
  #   data
  #   |> cast(attrs, [])
  #   |> cast_embed(:provided, required: true)
  #   |> cast_embed(:found, required: true)
  # end
end
