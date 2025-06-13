defmodule PoeSystemWeb.SseController do
  use PoeSystemWeb, :controller
  require Logger

  @allowed_topics ["build"]

  def subscribe(conn, params) do
    case get_allowed_topics(params) do
      t when is_list(t) and length(t) == 0 ->
        conn
        |> resp(400, "Bad request")

      topics when is_list(topics) ->
        Logger.debug(fn -> "subscribed to topics #{inspect(topics)}" end)
        # FIXME: if client reconnects right after build processed it does not
        # receives message about page reload
        SsePhoenixPubsub.stream(conn, {PoeSystem.PubSub, topics})
    end
  end

  defp get_allowed_topics(params) do
    get_topics(params)
    |> Enum.filter(fn t ->
      case String.split(t, ":", parts: 2) do
        [key, _] -> key in @allowed_topics
        [key] -> key in @allowed_topics
        _ -> false
      end
    end)
  end

  defp get_topics(%{"topics" => topics}) do
    case topics do
      str when is_binary(str) -> String.split(str, ",")
      nil -> []
    end
  end
end
