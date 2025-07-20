defmodule ItemStory do
  use PhoenixStorybook.Story, :component

  def function, do: &PoeSystemWeb.Components.item/1

  def template do
    """
      <div data-theme="light">
        <.psb-variation />
      </div>
    """
  end

  def variations do
    [
      %VariationGroup{
        id: :default,
        variations: [
          %Variation{
            id: :with_name,
          attributes: %{
            name: "New Item",
            basetype: "Some Axe",
            rarity: "normal",
            mods: [
              %{stat_id: "stat_id", text: "Some mod"}
            ]
          },

          },
          %Variation{
            id: :without_name,
            attributes: %{
              basetype: "Some Axe",
              rarity: "normal",
              mods: [
                %{stat_id: "stat_id", text: "Some mod"}
              ]
            },
          },
          %Variation{
            id: :custom_name,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "normal",
              mods: [
                %{stat_id: "stat_id", text: "Some mod"}
              ],
            },
            slots: [
              ~s|<:name_block><div>custom add to name</div></:name_block>|
            ]
          },
        ]
      },
      %VariationGroup{
        id: :rarity,
        variations: [
          %Variation{
            id: :normal,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "normal",
              mods: []
            }
          },
          %Variation{
            id: :magic,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "magic",
              mods: []
            }
          },
          %Variation{
            id: :rare,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "rare",
              mods: []
            }
          },
          %Variation{
            id: :unique,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "unique",
              mods: []
            }
          },
        ]
      },
      %VariationGroup{
        id: :mods,
        variations: [
          %Variation{
            id: :default_mods,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "normal",
              mods: [
                %{stat_id: "some stat_id", text: "Some mod"}
              ]
            },
          },
          %Variation{
            id: :custom_mods,
            attributes: %{
              name: "New Item",
              basetype: "Some Axe",
              rarity: "normal",
              mods: [
                %{stat_id: "some stat_id", text: "Some mod"}
              ]
            },
            slots: [
              """
                <:mods_block :let={mod}>customized mod: {mod.stat_id}</:mods_block>
              """
            ]
          },
        ]
      }
    ]
  end
  
end
