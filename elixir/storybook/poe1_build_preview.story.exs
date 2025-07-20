defmodule PoeSystemWeb.Poe1BuildPreviewStory do
  use PhoenixStorybook.Story, :example
  use PoeSystemWeb, :live_view
  import PoeSystemWeb.Components

  @items_data %{
        "helmet" => %{
          item: %{
            basetype: "New basetype",
            name: "New Item",
            rarity: "unique",
            mods: [
              %{stat_id: "stat_id1", text: "text1"},
              %{stat_id: "stat_id2", text: "text2"}
            ]
          },
          config: %{
            basetype: false,
            option: "Unique",
          }
        },
        "belt" => %{
          item: %{
            basetype: "New Belt",
            name: "New Belt",
            rarity: "unique",
            mods: [
              %{stat_id: "stat_id1", text: "text1"},
              %{stat_id: "stat_id2", text: "text2"}
            ]
          },
          config: %{
            basetype: false,
            option: nil,
          }
        },
        "amulet" => %{
          item: %{
            basetype: "New basetype amulet",
            name: "New Amulet",
            rarity: "unique",
            mods: [
              %{stat_id: "stat_id1", text: "text1"},
              %{stat_id: "stat_id2", text: "text2"}
            ]
          },
          config: %{
            basetype: false,
            option: %{
              "Mods" => %{
                "stat_id1" => "Exist",
                "stat_id2" => %{"Exact" => 10}
              }
            }
          }
        }
      }

  @impl true
  def mount(_params, _session, socket) do
    {
      :ok,
      socket
      |> assign(:items, @items_data)
    }
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div data-theme="light">
      <.form :let={f} for={%{}} phx-submit="submit-preview">
        <.input 
          type="select"
          label="Profile for items"
          phx-change="change-profile" 
          field={f[:profile]}
          options={[
            [key: "Choose profile", value: "", disabled: true, selected: true],
            [key: "Simple", value: "simpleeverything"],
            [key: "Simple no res", value: "simplenores"],
          ]}
        />
        <.button>Submit</.button>
      </.form>
      <div class="grid grid-cols-3 gap-4">
        <div :for={{key, %{item: item, config: config}} <- @items}>
          <.item_config_readonly item={item} config={config} />
        </div>
      </div> 
    </div>
    """
  end

  @impl true
  def handle_event("submit-preview", params, socket) do
    {:noreply, socket}
  end

  def handle_event("change-profile", params, socket) do
    IO.inspect(params)
    {:noreply, socket}
  end
end
