defmodule PoeSystem.BuildInfo.ItemWithConfig do
  alias PoeSystem.BuildInfo.RequiredItem
  use Ecto.Schema

  embedded_schema do
    embeds_one :item, RequiredItem
  end
end
