defmodule PoeSystemWeb.Poe1BuildCalcBuildLive do
  use PoeSystemWeb, :live_view
  import PoeSystemWeb.Components
  require Logger
  alias PoeSystem.{Build, Repo, BuildProcessing}

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  def zip_items(%Build{} = build) do
    found = Map.from_struct(build.found || %{})

    build.provided
    |> Map.from_struct()
    |> Enum.map(fn {k, p} -> {k, p, found[k]} end)
  end

  @impl true
  def render(assigns) do
    ~H"""
    <.button phx-click="recalculate">Recalculate</.button>
    <div class="grid grid-cols-2 gap-4 m-4">
      <.async_result :let={build} assign={@build}>
        <:failed>Failed to load</:failed>
        <%= for {k, p, f} <- zip_items(build) do %>
          <div :if={not is_list(p)}>
            <h1>{k}</h1>
            <.item_config_readonly
              :if={p}
              item={p.item}
              config={p.config}
            />
          </div>
          <div :if={is_list(p)}>
            <h1>{k}</h1>
            <div :for={d <- p} class="mb-2">
              <.item_config_readonly
                item={d.item}
                config={d.config}
              />
            </div>
          </div>
          <div :if={not is_list(f)}>
            <h1>{k}</h1>
            <.item_simple
              :if={f}
              item={f}
            />
          </div>
          <div :if={is_list(f)}>
            <h1>{k}</h1>
            <div :for={d <- f} class="mb-2">
              <.item_simple
                item={d}
              />
            </div>
          </div>
        <% end %>
      </.async_result>
    </div>
    """
  end

  @impl true
  def handle_params(%{"id" => id}, _uri, socket) do
    {:noreply,
     socket
     |> assign_async(:build, fn ->
       build = Repo.get!(Build, id)
       {:ok, %{build: build}}
     end)
     |> assign(:id, id)}
  end

  @impl true
  def handle_info({:new_pob, {pobdata, itemsets, skillsets}}, socket) do
    socket =
      socket
      |> assign(:pobdata, pobdata)
      |> assign(:itemsets, itemsets)
      |> assign(:skillsets, skillsets)

    {:noreply, push_patch(socket, to: ~p"/poe1/build-calc/preview")}
  end

  @impl true
  def handle_event("recalculate", _params, socket) do
    {:ok, _} = BuildProcessing.queue_processing_build(socket.assigns.id)
    {:noreply, socket}
  end
end
