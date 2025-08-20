defmodule PoeSystem.RemoveExcessiveBasetypes do
  require Logger
  alias Phoenix.PubSub
  alias PoeSystem.Repo
  alias PoeSystem.Items.{Item, NativeItem}
  use Oban.Worker, queue: :remove_basetypes
  use Telemetria
  import Ecto.Query

  @maximum_items 10_000

  @impl Oban.Worker
  def perform(job) do
    remove_items()
    :ok
  end

  @telemetria level: :info, group: :poe1_remove_basetypes
  def remove_items() do
    inner_query = from it in Item,
      select: %{basetype: it.basetype, item_id: it.item_id, rn: over(rank(), :basetype)},
      windows: [basetype: [partition_by: it.basetype, order_by: [desc: it.item_id]]]

    q = Item
    |> with_cte("rnk", as: ^inner_query)
    |> join(:inner, [it], r in "rnk", on: it.basetype == r.basetype and it.item_id == r.item_id)
    |> where([it, r], r.rn > @maximum_items)

    Repo.delete_all(q, timeout: :timer.minutes(5))
  end
end
