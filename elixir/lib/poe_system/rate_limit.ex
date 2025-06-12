defmodule PoeSystem.RateLimit do
  use Hammer, backend: :ets
end
