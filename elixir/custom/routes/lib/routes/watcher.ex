defmodule Routes.Watcher do
  use GenServer
  require Logger

  @moduledoc """
  A GenServer that watches for changes to router files and regenerates routes.

  This module provides functionality to:
  - Watch for file system changes in the current working directory
  - Detect modifications to router.ex files
  - Automatically regenerate routes when router files change
  """

  @doc """
  Child spec for supervision tree.
  """
  def child_spec(opts) do
    %{
      id: __MODULE__,
      start: {__MODULE__, :start_link, [opts]},
      type: :worker,
      restart: :permanent,
      shutdown: 500
    }
  end

  @doc """
  Starts the watcher process.

  ## Options

  Accepts GenServer options. The process will be registered under the module name.
  """
  def start_link(opts \\ []) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  @doc """
  Initializes the watcher by starting a FileSystem process and subscribing to events.
  """
  def init(_opts) do
    {:ok, watcher_pid} = FileSystem.start_link(dirs: [File.cwd!()])
    FileSystem.subscribe(watcher_pid)
    {:ok, %{watcher_pid: watcher_pid}}
  end

  @doc """
  Handles file system events, specifically looking for router.ex file modifications.
  """
  def handle_info({:file_event, _watcher_pid, {path, events}}, state) when is_list(events) do
    if router_file_modified?(path, events) do
      Logger.info("Router file changed: #{path}")
      regenerate_routes(path)
    end

    {:noreply, state}
  end

  def handle_info({:file_event, _watcher_pid, :stop}, state), do: {:noreply, state}
  def handle_info(_event, state), do: {:noreply, state}

  defp router_file_modified?(path, events) do
    String.ends_with?(path, "router.ex") and Enum.any?(events, &(&1 == :modified))
  end

  defp regenerate_routes(router_path) do
    router_module = get_router_module()

    try do
      Code.compile_file(router_path)
      {:ok, _routes} = Routes.Processor.process_routes(router_module)
      Logger.info("Routes successfully regenerated")
    rescue
      e ->
        Logger.error("Failed to regenerate routes: #{inspect(e)}")
    end
  end

  defp get_router_module do
    Application.get_env(:routes, :router) ||
      raise "No router module configured. Please add config :routes, router: YourAppWeb.Router"
  end
end
