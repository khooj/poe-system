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
      |> assign(:form, %{"profile" => "simpleeverything", "itemset" => List.first(itemsets), "skillset" => List.first(skillsets)} |> to_form())
      |> assign_async(:items, fn -> extract_items(pobdata, List.first(itemsets), List.first(skillsets)) end)

    {:ok, socket}
  end

  defp extract_items(pobdata, itemset, skillset, profile \\ "simpleeverything") do
    {:ok, build} = Native.extract_build_config(pobdata, itemset, skillset, profile)

    provided = build["provided"]
    |> Enum.map(fn 
        {k, v} when is_map(v) -> {k, NativeItem.from_json(v)} 
        {k, v} when is_list(v) -> {k, v |> Enum.map(&NativeItem.from_json/1)} 
        a -> a
      end)
    |> Enum.into(%{})

    {:ok, %{items: provided}}
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
          <div :for={{key, d} <- data |> filter_items()}>
            <h1>{key}</h1>
            <.item_config_readonly :if={d && is_struct(d)} item={d.item} config={d.config} />
          </div>
        </div>
        <p>gems</p>
        <div class="grid grid-cols-3 gap-4 mt-4">
          <div :for={d <- data["gems"]}>
            <.item_config_readonly item={d.item} config={d.config} />
          </div>
        </div>
        <p>flasks</p>
        <div class="grid grid-cols-3 gap-4 mt-4">
          <div :for={d <- data["flasks"]}>
            <.item_config_readonly item={d.item} config={d.config} />
          </div>
        </div>
        <p>jewels</p>
        <div class="grid grid-cols-3 gap-4 mt-4">
          <div :for={d <- data["jewels"]}>
            <.item_config_readonly item={d.item} config={d.config} />
          </div>
        </div>
      </.async_result>
    </div>
    """
  end

  defp filter_items(items) do
    items
      |> Enum.reject(fn 
        {k, v} when is_nil(v) -> true
        {"gems", _} -> true
        {"flasks", _} -> true
        {"jewels", _} -> true
        a -> false
    end)
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
