defmodule PoeSystem.BuildProcessing.Mods do
  alias PoeSystem.Items.{Item, ModStatId}
  alias PoeSystem.Items

  defp extract_mods_for_search(mods) do
    mods
      |> Enum.map(fn {%ModStatId{value: value}, _} -> value end)
  end

  defp append_query(q, %{option: nil}, _), do: q

  defp append_query(q, %{option: {:mods, mods}}, _) do
    q
    |> Items.append_mods(extract_mods_for_search(mods))
  end

  defp append_query(q, %{option: :unique}, %{item: item}) do
    q
    |> Items.append_name(item.name)
  end

  defp append_query(q, %{basetype: false}, _), do: q

  defp append_query(q, %{basetype: true}, %{item: item}) do
    q
    |> Items.append_basetype(item.basetype)
  end

  def extract_options_for_search(%{config: config} = item) do
    Item
    |> append_query(config, item)
  end
end
