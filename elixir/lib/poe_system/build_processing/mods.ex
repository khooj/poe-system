defmodule PoeSystem.BuildProcessing.Mods do
  alias PoeSystem.Items.{Item, ModStatId}
  alias PoeSystem.Items

  defp extract_mods_for_search(mods) do
    mods
    |> Enum.filter(fn 
      {_, :exist} -> true 
      {_, :exact} -> true 
      _ -> false
    end)
    |> Enum.map(fn {%ModStatId{value: value}, _} -> value end)
  end

  defp append_query_option(q, %{option: nil}, _), do: q

  defp append_query_option(q, %{option: {:mods, mods}}, _) do
    q
    |> Items.append_mods(extract_mods_for_search(mods))
  end

  defp append_query_option(q, %{option: :unique}, %{item: item}) do
    q
    |> Items.append_name(item.name)
  end

  defp append_query_basetype(q, %{basetype: false}, _), do: q

  defp append_query_basetype(q, %{basetype: true}, %{item: item}) do
    q
    |> Items.append_basetype(item.basetype)
  end

  def extract_options_for_search(%{config: config} = item) do
    Item
    |> append_query_option(config, item)
    |> append_query_basetype(config, item)
  end

  def unique?(%Item{rarity: "unique"}), do: true
  def unique?(%Item{}), do: false

  def gem?(%Item{category: :gems}), do: true
  def gem?(%Item{}), do: false
end
