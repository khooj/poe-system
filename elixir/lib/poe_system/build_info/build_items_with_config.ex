defmodule PoeSystem.BuildInfo.BuildItemsWithConfig do
  use Ecto.Schema
  import Ecto.Changeset

  embedded_schema do
    embeds_one :helmet, ItemWithConfig
    embeds_one :body, ItemWithConfig
    embeds_one :boots, ItemWithConfig
    embeds_one :gloves, ItemWithConfig
    embeds_one :weapon1, ItemWithConfig
    embeds_one :weapon2, ItemWithConfig
    embeds_one :ring1, ItemWithConfig
    embeds_one :ring2, ItemWithConfig
    embeds_one :belt, ItemWithConfig
    embeds_many :flasks, ItemWithConfig
    embeds_many :gems, ItemWithConfig
    embeds_many :jewels, ItemWithConfig
  end
end
