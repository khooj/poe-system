defmodule PoeSystemWeb.Plug.CSP do
  import Plug.Conn

  def put_content_security_policy(conn, fun) when is_function(fun, 1) do
    put_content_security_policy(conn, fun.(conn))
  end

  def put_content_security_policy(conn, opts) when is_list(opts) do
    csp =
      opts
      |> Keyword.has_key?(:default_src)
      |> case do
        false -> [default_src: "'self'"] ++ opts
        true -> opts
      end
      |> Enum.reduce([], fn {name, sources}, acc ->
        sources = List.wrap(sources)

        Keyword.update(acc, name, sources, &(&1 ++ sources))
      end)
      |> Enum.reduce("", fn {name, sources}, acc ->
        name = String.replace(to_string(name), "_", "-")

        sources =
          sources
          |> Enum.uniq()
          |> Enum.join(" ")
          |> String.replace("'nonce'", "'nonce-#{get_csp_nonce()}'")

        "#{acc}#{name} #{sources};"
      end)

    put_resp_header(conn, "content-security-policy", csp)
  end

  def get_csp_nonce do
    if nonce = Process.get(:plug_csp_nonce) do
      nonce
    else
      nonce = csp_nonce()
      Process.put(:plug_csp_nonce, nonce)
      nonce
    end
  end

  defp csp_nonce do
    24
    |> :crypto.strong_rand_bytes()
    |> Base.encode64(padding: false)
  end
end
