defmodule PoeSystemWeb.RateLimit do
  use Hammer, backend: :ets
end
