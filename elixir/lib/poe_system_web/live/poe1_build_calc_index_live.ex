defmodule PoeSystemWeb.Poe1BuildCalcIndexLive do
  use PoeSystemWeb, :live_view
  alias RustPoe.Native
  require Logger

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex justify-center">
      <.form :let={f} for={%{}} as={:pob} phx-submit="submit" class="w-xl">
        <.input
          field={f[:pob]}
          id="pobdata"
          label="Path of Building data"
          placeholder="base64-encoded string"
          type="textarea"
        />
        <.button patch={~p"/poe1/build-calc/preview"} phx-disable-with="Loading items...">Save</.button>
      </.form>
    </div>
    """
  end

  @impl true
  def handle_event("submit", %{"pob" => %{"pob" => pobdata}}, socket) do
    {:ok, itemsets, skillsets} = Native.get_itemsets_skillsets(pobdata)
    {:noreply, socket}
  end
end
