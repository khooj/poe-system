defmodule PoeSystem.Items do
  import Ecto.Query
  alias PoeSystem.Repo
  alias PoeSystem.Items.Item
  alias Utils

  @type search_items_opt() ::
          {:basetype, String.t()}
          | {:category, String.t()}
          | {:subcategory, String.t()}

  @type mods() :: [String.t()]

  # TODO: using limit slows query, probably because of incorrect execution planning
  # due to using jsonb field
  @spec search_items_by_attrs_query([mods()], [search_items_opt()]) :: Ecto.Query.t()
  def search_items_by_attrs_query(item_mods, opts \\ []) do
    basetype = Keyword.get(opts, :basetype)
    category = Keyword.get(opts, :category)
    subcategory = Keyword.get(opts, :subcategory)

    Item
    |> opt(basetype, &where(&1, [m], m.basetype == ^basetype))
    |> opt(category, &where(&1, [m], m.category == ^category))
    |> opt(subcategory, &where(&1, [m], m.subcategory == ^subcategory))
    |> append_mods(item_mods)
    |> order_by([m], m.id)
  end

  def append_flask_quality(q, quality) do
    q
    |> where([m], m.category == :flasks)
    |> where([m], fragment("(?->>'quality')::int", m.info) >= ^quality)
  end

  def append_id_cursor(query, nil), do: query

  def append_id_cursor(query, id) do
    query
    |> where([m], m.item_id > ^id)
    |> order_by([m], m.item_id)
  end

  def append_mods(q, mods) do
    q
    |> where([m], fragment("jsonb_path_query_array(?, '$.mods[*].stat_id') \\?& ?", m.info, ^mods))
  end

  def append_name(q, name) do
    q
    |> where([m], m.name == ^name)
  end

  def append_basetype(q, basetype) do
    q
    |> where([m], m.basetype == ^basetype)
  end

  defp opt(q, nil, _), do: q
  defp opt(q, false, _), do: q

  defp opt(q, _, fr) do
    q |> fr.()
  end
end
