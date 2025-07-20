defmodule PoeSystem.BuildProcessing.Mods do
  alias PoeSystem.Items.Item
  alias PoeSystem.Items
  import Ecto.Query

  defp extract_mods_for_search(%{"Mods" => mods}) do
    mods
    |> Map.keys()
  end

  defp unique?("Unique"), do: true
  defp unique?(_), do: false

  defp append_query(q, %{"option" => nil}, _), do: q

  defp append_query(q, %{"option" => %{"Mods" => _} = mods}, item) do
    q
    |> Items.append_mods(extract_mods_for_search(mods))
  end

  defp append_query(q, %{"option" => "Unique"}, %{"item" => item}) do
    q
    |> Items.append_name(item["name"])
  end

  defp append_query(q, %{"basetype" => false}, _), do: q

  defp append_query(q, %{"basetype" => true}, %{"item" => item}) do
    q
    |> Items.append_basetype(item["basetype"])
  end

  def extract_options_for_search(%{"config" => config} = item) do
    Item
    |> append_query(config, item)
  end
end
