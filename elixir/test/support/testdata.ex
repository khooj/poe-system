defmodule PoeSystem.Testdata do
  @testdata_dir Path.join([__DIR__, "../testdata"])
  @config_itemset "Midgame"
  @config_skillset "Maps"

  def pobdata_file() do
    String.trim(File.read!(Path.join([@testdata_dir, "pob.txt"])))
  end

  def extract_config() do
    {:ok, data} =
      RustPoe.Native.extract_build_config(pobdata_file(), @config_itemset, @config_skillset)

    data
  end

  def config_info do
    {@config_itemset, @config_skillset}
  end

  def items do
    RustPoe.Native.get_items_from_stash_data(File.read!(Path.join([@testdata_dir, "stash.json"])))
  end
end
