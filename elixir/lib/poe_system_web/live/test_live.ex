defmodule PoeSystemWeb.TestLive do
  use PoeSystemWeb, :live_view
  require Logger

  @impl true
  def render(assigns) do
    ~H"""
    <div class="hero bg-base-200 min-h-screen">
      <div class="hero-content text-center">
        <div class="max-w-md">
          <h1 class="text-5xl font-bold">
            <.icon name="hero-x-mark-solid" /> hello there {@temp} criminal scum
          </h1>
          <p class="py-6">
            Lorum ipsum tatata ahhahah
          </p>
          <button class="btn btn-primary" phx-click="test">Inc temp</button>
        </div>
      </div>
    </div>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok, assign(socket, :temp, 50)}
  end

  @impl true
  def handle_event("test", _params, socket) do
    {:noreply, update(socket, :temp, &(&1 + 1))}
  end
end
