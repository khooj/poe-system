defmodule PoeSystemWeb.Poe1BuildIndexStory do
  use PhoenixStorybook.Story, :example
  use PoeSystemWeb, :live_view
  require Logger
  alias RustPoe.Native

  def render(assigns) do
    ~H"""
      <.form :let={f} for={%{}} as={:pob} phx-submit="submit" class="w-full" data-theme="light">
        <.input
          field={f[:pob]}
          id="pobdata"
          label="Path of Building data"
          placeholder="base64-encoded string"
          type="textarea"
          phx-change="validate-pob"
          phx-debounce="300"
        />
          <.async_result :if={assigns[:itemsets]} :let={itemsets} assign={@itemsets}>
            <:loading><.loading color="primary" />Loading itemsets</:loading>
            <:failed :let={_failure}>Error loading itemsets</:failed>
            <div>
              <.select>
                <option :for={item <- itemsets}>{item}</option>
              </.select>
            </div>
          </.async_result>
          <.async_result :if={assigns[:skillsets]} :let={skillsets} assign={@skillsets}>
            <:loading><.loading color="primary" />Loading skillsets</:loading>
            <:failed :let={_failure}>Error loading skillsets</:failed>
            <div>
              <.select>
                <option :for={item <- skillsets}>{item}</option>
              </.select>
            </div>
          </.async_result>
          <.button>Save</.button>
      </.form>
    """
  end

  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  def handle_event("test", _params, socket) do
    Logger.debug("test event")
    {:noreply, socket}
  end

  def handle_event("validate-pob", %{"pob" => %{"pob" => pobdata}}, socket) do
    socket = socket
      |> assign_async([:itemsets, :skillsets], fn -> 
        {:ok, itemsets, skillsets} = Native.get_itemsets_skillsets(pobdata)
        {:ok, %{itemsets: itemsets, skillsets: skillsets}}
      end)
    {:noreply, socket}
  end

  def handle_event("submit", params, socket) do
    {:noreply, socket}
  end
end
