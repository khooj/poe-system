defmodule PoeSystemWeb.LiveComponents.PreviewPob do
  use PoeSystemWeb, :live_component
  import PoeSystemWeb.Components
  alias RustPoe.Native
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
      |> assign_async(:items, fn -> 
        {:ok, build} = Native.extract_build_config(pobdata, List.first(itemsets), List.first(skillsets))

        provided = build["provided"]
        |> Enum.map(fn 
            {k, v} when is_map(v) -> {k, NativeItem.from_json(v)} 
            {k, v} when is_list(v) -> {k, v |> Enum.map(&NativeItem.from_json/1)} 
          end)
        |> Enum.into(%{})

        {:ok, %{items: provided}}
      end)

    {:ok, socket}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="flex flex-col justify-center">
      <div>
        <.form :let={f} for={%{}} phx-submit="submit-preview">
          <.input 
            type="select"
            label="Profile for items"
            phx-change="change-profile" 
            phx-target={@myself}
            field={f[:profile]}
            options={[
              [key: "Choose profile", value: "", disabled: true, selected: true],
              [key: "Simple", value: "simpleeverything"],
              [key: "Simple no res", value: "simplenores"],
            ]}
          />
          <.button>Submit</.button>
        </.form>
      </div>
      <.async_result :let={data} assign={@items}>
        <:loading>Loading items...</:loading>
        <:failed>Failed to load</:failed>
        <div class="grid grid-cols-3 gap-4">
          <div :for={{key, d} <- data}>
            <.item_config_readonly :if={is_struct(d)} item={d.item} config={d.config} />
          </div>
        </div> 
      </.async_result>
    </div>
    """
  end

  def handle_event("change-profile", %{"profile" => profile}, socket) do
    %{
      pobdata: pobdata,
      itemsets: itemsets,
      skillsets: skillsets
    } = socket.assigns

    socket = socket
      |> assign_async(:items, fn -> 
        {:ok, build} = Native.extract_build_config(pobdata, List.first(itemsets), List.first(skillsets), profile)
        provided = build["provided"]
        |> Enum.map(fn 
            {k, v} when is_map(v) -> {k, NativeItem.from_json(v)} 
            {k, v} when is_list(v) -> {k, v |> Enum.map(&NativeItem.from_json/1)} 
          end)
        |> Enum.into(%{})
        {:ok, %{items: provided}}
      end)
      # |> assign(:items, AsyncResult.loading())

    {:noreply, socket}
  end
end
