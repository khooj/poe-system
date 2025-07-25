defmodule PoeSystem.Testdata do
  alias PoeSystem.Items.Item
  alias PoeSystem.Items
  alias PoeSystem.Build
  alias PoeSystem.Repo
  alias Ecto.UUID
  @testdata_dir Path.join([__DIR__, "../testdata"])
  @config_itemset "Midgame"
  @config_skillset "Maps"

  def pobdata_file() do
    :rarity
    String.trim(File.read!(Path.join([@testdata_dir, "pob.txt"])))
  end

  def extract_config(opts \\ []) do
    itemset =
      if Keyword.get(opts, :early_setup, false) do
        "Level 13 example"
      else
        @config_itemset
      end

    {:ok, data} =
      RustPoe.Native.extract_build_config(pobdata_file(), itemset, @config_skillset)

    data
  end

  def config_info do
    {@config_itemset, @config_skillset}
  end

  def stash_json do
    File.read!(Path.join([@testdata_dir, "stash.json"]))
  end

  def items do
    RustPoe.Native.get_items_from_stash_data(stash_json())
  end

  def insert_items do
    for item <- Items.into_elixir_items(items()) do
      Item.changeset(item)
      |> Repo.insert_or_update!()
    end
  end

  def insert_build do
    {itemset, skillset} = config_info()

    Build.changeset(%Build{}, %{
      id: UUID.bingenerate(),
      itemset: itemset,
      skillset: skillset,
      pob: pobdata_file(),
      data: extract_config()
    })
    |> Repo.insert!()
  end
end
