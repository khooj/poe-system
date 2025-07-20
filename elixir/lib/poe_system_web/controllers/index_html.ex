defmodule PoeSystemWeb.IndexHTML do
  @moduledoc """
  This module contains pages rendered by PageController.

  See the `page_html` directory for all templates available.
  """
  use PoeSystemWeb, :html

  embed_templates "index_html/*"
end
