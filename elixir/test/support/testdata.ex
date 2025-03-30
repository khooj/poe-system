defmodule PoeSystem.Testdata do
  def pobdata_file() do
    String.trim(File.read!(Path.join([__DIR__, "../testdata/pob.txt"])))
  end

  def extract_config() do
    {:ok, data} = RustPoe.Native.extract_build_config(pobdata_file(), "Level 13 example", "Maps")
    data
  end
end
