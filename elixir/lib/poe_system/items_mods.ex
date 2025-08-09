defmodule PoeSystem.ItemsMods do
  use Ecto.Schema
  import Ecto.Changeset
  import Ecto.Query
  alias PoeSystem.Repo
  alias PoeSystem.Items.Item
  alias __MODULE__

  @primary_key false
  schema "items_mods" do
    field :item_id, :integer
    field :mods, {:array, :string}
  end
end
