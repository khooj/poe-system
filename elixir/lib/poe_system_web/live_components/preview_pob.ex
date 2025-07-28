defmodule PoeSystemWeb.LiveComponents.PreviewPob do
  use PoeSystemWeb, :live_component
  import PoeSystemWeb.Components
  alias RustPoe.NativeWrapper
  alias Phoenix.LiveView.AsyncResult
  alias PoeSystem.Items.NativeItem

  def update(assigns, socket) do
    %{
      pobdata: pobdata,
      itemsets: itemsets,
      skillsets: skillsets
    } = assigns

    socket = socket
      |> assign(:pobdata, pobdata)
      |> assign(:itemsets, itemsets)
      |> assign(:skillsets, skillsets)
      |> assign(:form, %{"profile" => "simpleeverything", "itemset" => List.first(itemsets), "skillset" => List.first(skillsets)} |> to_form())
      |> assign_async(:items, fn -> extract_items(pobdata, List.first(itemsets), List.first(skillsets)) end)

    {:ok, socket}
  end

  defp extract_items(pobdata, itemset, skillset, profile \\ "simpleeverything") do
    {:ok, build} = NativeWrapper.extract_build_config(pobdata, itemset, skillset, profile)

    {:ok, %{items: build.provided}}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col justify-center">
      <div>
        <.form for={@form} phx-submit="submit-preview" phx-change="change-options" phx-target={@myself}>
          <.input 
            type="select"
            label="Profile for items"
            field={@form[:profile]}
            options={[
              [key: "Simple", value: "simpleeverything"],
              [key: "Simple no res", value: "simplenores"],
            ]}
          />
          <.input 
            type="select"
            label="Itemset"
            field={@form[:itemset]}
            options={@itemsets}
          />
          <.input 
            type="select"
            label="Skillset"
            field={@form[:skillset]}
            options={@skillsets}
          />
          <div class="flex gap-4 items-center">
            <.button>Submit</.button>
            <div :if={@items.loading}><.loading color="primary" />Loading</div>
          </div>
        </.form>
      </div>
      <.async_result :let={data} assign={@items}>
        <:failed>Failed to load</:failed>
        <div class="grid grid-cols-3 gap-4">
          <div>
            <h1>amulet</h1>
            <.item_config_readonly :if={data.amulet} item={data.amulet.item} config={data.amulet.config} />
          </div>
          <div>
            <h1>helmet</h1>
            <.item_config_readonly :if={data.helmet} item={data.helmet.item} config={data.helmet.config} />
          </div>
          <div>
            <h1>body</h1>
            <.item_config_readonly :if={data.body} item={data.body.item} config={data.body.config} />
          </div>
          <div>
            <h1>boots</h1>
            <.item_config_readonly :if={data.boots} item={data.boots.item} config={data.boots.config} />
          </div>
          <div>
            <h1>gloves</h1>
            <.item_config_readonly :if={data.gloves} item={data.gloves.item} config={data.gloves.config} />
          </div>
          <div>
            <h1>weapon1</h1>
            <.item_config_readonly :if={data.weapon1} item={data.weapon1.item} config={data.weapon1.config} />
          </div>
          <div>
            <h1>weapon2</h1>
            <.item_config_readonly :if={data.weapon2} item={data.weapon2.item} config={data.weapon2.config} />
          </div>
          <div>
            <h1>ring1</h1>
            <.item_config_readonly :if={data.ring1} item={data.ring1.item} config={data.ring1.config} />
          </div>
          <div>
            <h1>ring2</h1>
            <.item_config_readonly :if={data.ring2} item={data.ring2.item} config={data.ring2.config} />
          </div>
          <div>
            <h1>belt</h1>
            <.item_config_readonly :if={data.belt} item={data.belt.item} config={data.belt.config} />
          </div>
        </div>
        <p>gems</p>
        <div class="grid grid-cols-3 gap-4 mt-4">
          <div :for={d <- data.gems}>
            <.item_config_readonly item={d.item} config={d.config} />
          </div>
        </div>
        <p>flasks</p>
        <div class="grid grid-cols-3 gap-4 mt-4">
          <div :for={d <- data.flasks}>
            <.item_config_readonly item={d.item} config={d.config} />
          </div>
        </div>
        <p>jewels</p>
        <div class="grid grid-cols-3 gap-4 mt-4">
          <div :for={d <- data.jewels}>
            <.item_config_readonly item={d.item} config={d.config} />
          </div>
        </div>
      </.async_result>
    </div>
    """
  end

  defp keys() do
    [
      :amulet,
      :helmet,
      :body,
      :boots,
      :gloves,
      :weapon1,
      :weapon2,
      :ring1,
      :ring2,
      :belt
    ]
  end

  def handle_event("change-options", params, socket) do
    %{
      pobdata: pobdata,
    } = socket.assigns

    %{
      "profile" => profile,
      "itemset" => itemset,
      "skillset" => skillset,
    } = params

    socket = socket
      |> assign(:form, to_form(params))
      |> assign_async(:items, fn -> 
        # Process.sleep(:timer.seconds(1))
        extract_items(pobdata, itemset, skillset, profile) 
      end)

    {:noreply, socket}
  end
end
