if Code.ensure_loaded?(WebSockex) do
  defmodule PoeSystemWeb.Plug.WebsocketProxy do
    alias Ecto.UUID
    @behaviour Phoenix.Socket.Transport

    require Logger
    import Phoenix.PubSub

    defmodule PoeSystemWeb.Plug.WebsocketProxy.WS do
      alias Phoenix.PubSub
      use WebSockex
      require Logger

      def start_link(opts) do
        url = Keyword.fetch!(opts, :url)
        topic = Keyword.fetch!(opts, :topic)
        Logger.debug(url)
        Logger.debug(topic)
        WebSockex.start_link(url, __MODULE__, topic)
      end

      def handle_connect(conn, state) do
        Logger.debug("ws conn #{state}")
        PubSub.subscribe(PoeSystem.PubSub, state)
        {:ok, state}
      end

      def terminate(reason, state) do
        PubSub.unsubscribe(PosSystem.PubSub, state)
        exit(:normal)
      end

      def handle_info(msg, state) do
        Logger.debug("got msg: #{msg}")
        PubSub.broadcast!(PoeSystem.PubSub, state, {:ws_msg, msg})
        {:ok, state}
      end
    end

    def child_spec(_opts) do
      {DynamicSupervisor, strategy: :one_for_one, name: :internal_ws_ds}
    end

    def connect(state) do
      {:ok, state}
    end

    def init(state) do
      connect_info = Keyword.fetch!(state.options, :connect_info)
      proxy_to = Keyword.fetch!(connect_info, :proxy_to)
      prefix = Keyword.fetch!(connect_info, :prefix)
      IO.inspect(state)

      uri =
        URI.new!(proxy_to)
        |> URI.append_path(prefix)
        |> URI.append_query(URI.encode_query(state.params))

      topic = UUID.generate()

      {:ok, pid} =
        DynamicSupervisor.start_child(
          :internal_ws_ds,
          {PoeSystemWeb.Plug.WebsocketProxy.WS, url: URI.to_string(uri), topic: topic}
        )

      PubSub.subscribe(PoeSystem.PubSub, topic)

      state =
        []
        |> Keyword.put(:topic, topic)
        |> Keyword.put(:pid, pid)

      {:ok, state}
    end

    def handle_in({msg, _opts}, state) do
      pid = get_pid(state)
      WebSockex.send_frame(pid, {:text, msg})
      {:ok, state}
    end

    def handle_info({:ws_msg, msg}, state) do
      {:push, {:text, msg}, state}
    end

    def terminate(reason, state) do
      topic = get_topic(state)
      PubSub.unsubscribe(PoeSystem.PubSub, topic)
      pid = get_pid(state)
      Process.exit(pid, :normal)
      exit(:normal)
    end

    defp get_topic(state) do
      Keyword.fetch!(state, :topic)
    end

    defp get_pid(state) do
      Keyword.fetch!(state, :pid)
    end
  end
end
