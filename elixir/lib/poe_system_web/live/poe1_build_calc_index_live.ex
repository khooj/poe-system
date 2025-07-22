defmodule PoeSystemWeb.Poe1BuildCalcIndexLive do
  use PoeSystemWeb, :live_view
  alias RustPoe.Native
  alias PoeSystemWeb.LiveComponents.{PobReceive, PreviewPob}
  require Logger

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
      <.live_component module={PobReceive} id="pob-receive" :if={@live_action == :new} />
      <%= if @live_action == :preview do %>
        <.live_component 
          module={PreviewPob} 
          id="preview-pob" 
          pobdata={@pobdata}
          itemsets={@itemsets}
          skillsets={@skillsets}
        />
      <% end %>
    """
  end

  def handle_params(_params, _uri, socket) do
    {:noreply, socket}
  end

  def handle_info({:new_pob, {pobdata, itemsets, skillsets}}, socket) do
    socket = socket
      |> assign(:pobdata, pobdata)
      |> assign(:itemsets, itemsets)
      |> assign(:skillsets, skillsets)

    {:noreply, push_patch(socket, to: ~p"/poe1/build-calc/preview")}
  end
end
