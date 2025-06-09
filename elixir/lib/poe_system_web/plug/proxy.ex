defmodule PoeSystemWeb.Plug.Proxy do
  import Plug.Conn
  alias Req
  require Logger

  def init(opts), do: opts

  def call(conn, opts) do
    redirect_to = Keyword.fetch!(opts, :redirect_to)
    match_path = Keyword.fetch!(opts, :match_path)
    plug = Keyword.get(opts, :plug, nil)

    if String.starts_with?(conn.request_path, match_path) do
      req = Req.new(base_url: redirect_to, plug: plug)

      req =
        conn.req_headers
        |> Enum.reduce(req, fn {k, v}, req ->
          Req.Request.put_new_header(req, k, v)
        end)

      resp =
        Req.get!(req, url: conn.request_path, decode_body: false)

      conn
      |> then(
        &Enum.reduce(Req.get_headers_list(resp), &1, fn {k, v}, conn ->
          put_resp_header(conn, k, v)
        end)
      )
      |> send_resp(resp.status, resp.body)
      |> halt()
    else
      conn
    end
  end
end
