defmodule PoeSystem.Build.BuildInfo do
  use Ecto.Schema
  import Ecto.Changeset
  alias PoeSystem.Build.{ProvidedItems, FoundItems}

  @type t :: %__MODULE__{}

  @primary_key false
  embedded_schema do
    embeds_one :provided, ProvidedItems
    embeds_one :found, FoundItems
  end

  def changeset(build_info, attrs \\ %{}) do
    build_info
    |> cast(attrs, [])
    |> cast_embed(:provided)
    |> cast_embed(:found)
  end
end
