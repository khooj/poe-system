defmodule ItemConfigStory do
  use PhoenixStorybook.Story, :component

  def function, do: &PoeSystemWeb.Components.item_config/1
  def imports, do: []

  def template do
    """
      <div data-theme="light">
        <.psb-variation />
      </div>
    """
  end

  def variations do
    [
      %Variation{
        id: :unique,
        attributes: %{
          item: %{
            name: "New Item",
            basetype: "Some Axe",
            rarity: "normal",
            mods: [
              %{stat_id: "stat_id", text: "Some mod"},
              %{stat_id: "stat_id", text: "Some mod"},
              %{stat_id: "stat_id", text: "Some mod"},
              %{stat_id: "stat_id", text: "Some mod"},
            ],
          },
          config: %{
            basetype: false,
            option: "Unique"
          }
        }
      },
      %Variation{
        id: :mods_readonly,
        attributes: %{
          item: %{
            name: "New Item",
            basetype: "Some Axe",
            rarity: "normal",
            mods: [
              %{stat_id: "stat_id", text: "Some mod"},
              %{stat_id: "stat_id2", text: "Some mod"},
              %{stat_id: "stat_id3", text: "Some mod"},
              %{stat_id: "stat_id4", text: "Some mod"},
            ],
          },
          config: %{
            basetype: false,
            option: %{
              "Mods" => %{
                "stat_id" => %{"Exact" => 32},
                "stat_id2" => "Ignore",
                "stat_id3" => %{"Range" => %{"start" => 0, "end" => 10}},
                "stat_id4" => "Exist",
              }
            }
          }
        }
      }
    ]
  end
  
end
