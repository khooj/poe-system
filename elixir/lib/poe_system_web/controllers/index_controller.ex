defmodule PoeSystemWeb.IndexController do
  use PoeSystemWeb, :controller

  def index(conn, _props) do
    conn
    |> render(:index)
  end
end
