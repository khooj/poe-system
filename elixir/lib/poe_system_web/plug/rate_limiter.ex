defmodule PoeSystemWeb.Plug.RateLimiter do
  import Plug.Conn
  require Logger
  alias PoeSystem.RateLimit

  def init(opts), do: opts

  def call(conn, _opts) do
    case RateLimit.hit(conn.remote_ip, :timer.seconds(1), 10) do
      {:allow, _} -> conn
      {:deny, _} -> conn |> send_resp(429, "Too Many Requests")
    end
  end
end
