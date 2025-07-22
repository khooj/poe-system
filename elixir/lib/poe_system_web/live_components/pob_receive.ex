defmodule PoeSystemWeb.LiveComponents.PobReceive do
  use PoeSystemWeb, :live_component
  alias RustPoe.Native

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex justify-center">
      <.form :let={f} for={%{}} as={:pob} phx-submit="submit" phx-target={@myself} class="w-xl">
        <.input
          field={f[:pob]}
          id="pobdata"
          label="Path of Building data"
          placeholder="base64-encoded string"
          type="textarea"
        />
        <.button phx-disable-with="Loading items...">Save</.button>
      </.form>
    </div>
    """
  end

  @impl true
  def handle_event("submit", %{"pob" => %{"pob" => pobdata}}, socket) do
    {:ok, itemsets, skillsets} = Native.get_itemsets_skillsets(pobdata)
    send(self(), {:new_pob, {pobdata, itemsets, skillsets}})
    {:noreply, socket}
  end
end
