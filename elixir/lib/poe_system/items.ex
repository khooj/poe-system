defmodule PoeSystem.Items do
  import Ecto.Query
  alias PoeSystem.Repo
  alias PoeSystem.Items.Item

  @type search_items_opt() ::
          {:basetype, String.t()}
          | {:category, String.t()}
          | {:subcategory, String.t()}

  @type mods() :: %{stat_id: String.t()}

  @spec search_items_by_attrs([search_items_opt()]) :: [%Item{}]
  def search_items_by_attrs(item_mods, opts \\ []) do
    search_items_by_attrs_query(item_mods, opts)
    |> Repo.all()
  end

  @spec search_items_by_attrs_query([search_items_opt()]) :: %Ecto.Query{}
  def search_items_by_attrs_query(item_mods, opts \\ []) do
    basetype = Keyword.get(opts, :basetype)
    category = Keyword.get(opts, :category)
    subcategory = Keyword.get(opts, :subcategory)

    mods =
      item_mods
      |> Enum.filter(& &1["stat_id"])
      |> Enum.map(fn b -> %{stat_id: b["stat_id"]} end)
      |> then(&%{mods: &1})

    Item
    |> opt(basetype, &where(&1, [m], m.basetype == ^basetype))
    |> opt(category, &where(&1, [m], m.category == ^category))
    |> opt(subcategory, &where(&1, [m], m.subcategory == ^subcategory))
    |> where([m], fragment("? @> ?", m.data, ^mods))
    |> order_by([m], m.id)
  end

  def append_id_cursor(query, nil), do: query

  def append_id_cursor(query, id) do
    query
    |> where([m], m.id > ^id)
  end

  defp opt(q, nil, _), do: q
  defp opt(q, false, _), do: q

  defp opt(q, _, fr) do
    q |> fr.()
  end
end
