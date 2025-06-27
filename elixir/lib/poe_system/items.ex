defmodule PoeSystem.Items do
  import Ecto.Query
  alias PoeSystem.Repo
  alias PoeSystem.Items.Item
  alias Utils

  def into_elixir_items(items) when is_list(items) do
    items
    |> Enum.map(&into_elixir_items/1)
  end

  def into_elixir_items(item) when is_map(item) do
    res =
      item
      |> Map.put("item_id", item["id"])
      |> Map.delete("id")
      |> Utils.to_atom_key_map()

    struct(Item, res)
  end

  def into_native_items(items) when is_list(items) do
    items
    |> Enum.map(&into_native_items/1)
  end

  def into_native_items(item) when is_struct(item) do
    item
    |> Utils.to_string_key_map()
    |> Map.put("id", item.item_id)
    |> Map.delete("item_id")
  end

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
      |> Enum.map(fn b -> b["stat_id"] end)

    Item
    |> opt(basetype, &where(&1, [m], m.basetype == ^basetype))
    |> opt(category, &where(&1, [m], m.category == ^category))
    |> opt(subcategory, &where(&1, [m], m.subcategory == ^subcategory))
    |> where([m], fragment("?->'mods'->'stat_id' \\?| ?", m.info, ^mods))
    |> order_by([m], m.id)
  end

  def search_gems_by_attrs_query(name, quality, level) do
    Item
    |> where([m], m.basetype == ^name)
    |> where([m], m.subcategory == "Gem")
    |> where([m], fragment("(?->>'level')::int", m.info) >= ^level)
    |> where([m], fragment("(?->>'quality')::int", m.info) >= ^quality)
    |> order_by([m], m.id)
  end

  # def search_flasks_by_attrs_query(name, quality, level) do
  #   n = "#{name}%"
  #
  #   Item
  #   |> where([m], ilike(m.basetype, ^name))
  #   |> where([m], m.subcategory == "Gem")
  #   |> where([m], fragment("(?->'level')::int", m.data) >= ^level)
  #   |> where([m], fragment("(?->'quality')::int", m.data) >= ^quality)
  #   |> order_by([m], m.id)
  # end

  def append_flask_quality(q, quality) do
    q
    |> where([m], m.category == "Flasks")
    |> where([m], fragment("(?->>'quality')::int", m.info) >= ^quality)
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
