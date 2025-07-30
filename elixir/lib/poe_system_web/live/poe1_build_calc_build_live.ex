defmodule PoeSystemWeb.Poe1BuildCalcBuildLive do
  use PoeSystemWeb, :live_view
  import PoeSystemWeb.Components
  require Logger
  alias PoeSystem.{Build, Repo, BuildProcessing}

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <.button phx-click="recalculate">Recalculate</.button>
    <div class="grid grid-cols-2 gap-4 m-4">
      <div>
        <p>Provided</p>
      <.async_result :let={data} assign={@provided}>
        <:failed>Failed to load</:failed>
        <div class="flex flex-col">
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
          <p>gems</p>
          <div>
            <div :for={d <- data.gems}>
              <.item_config_readonly item={d.item} config={d.config} />
            </div>
          </div>
          <p>flasks</p>
          <div>
            <div :for={d <- data.flasks}>
              <.item_config_readonly item={d.item} config={d.config} />
            </div>
          </div>
          <p>jewels</p>
          <div>
            <div :for={d <- data.jewels}>
              <.item_config_readonly item={d.item} config={d.config} />
            </div>
          </div>
        </div>
      </.async_result>
      </div>
      <div>
        <p>Found</p>
      </div>
    </div>
    """
  end

  @impl true
  def handle_params(%{"id" => id}, _uri, socket) do
    {:noreply, 
      socket 
      |> assign_async([:provided, :found, :build], fn -> 
        build = Repo.get!(Build, id) 
        {:ok, %{provided: build.provided, found: build.found, build: build}}
        end)
      |> assign(:id, id)
    }
  end

  @impl true
  def handle_info({:new_pob, {pobdata, itemsets, skillsets}}, socket) do
    socket = socket
      |> assign(:pobdata, pobdata)
      |> assign(:itemsets, itemsets)
      |> assign(:skillsets, skillsets)

    {:noreply, push_patch(socket, to: ~p"/poe1/build-calc/preview")}
  end

  @impl true
  def handle_event("recalculate", _params, socket) do
    {:ok,_} = BuildProcessing.queue_processing_build(socket.assigns.id)
    {:noreply, socket}
  end
end
