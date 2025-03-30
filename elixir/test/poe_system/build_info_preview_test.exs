defmodule PoeSystem.BuildInfoPreviewTest do
  alias PoeSystem.BuildInfoPreview
  alias PoeSystem.Testdata
  use ExUnit.Case
  use PoeSystemWeb.ConnCase

  test "check data after saving to db" do
    cfg = Testdata.extract_config()
    assert {:ok, data} = BuildInfoPreview.add_build(cfg, "itemset1", "skillset1", "randompob")
    assert data_after_db = BuildInfoPreview.get_build(data.id)
    assert data.data == data_after_db.data
    assert data.data == cfg
  end
end
