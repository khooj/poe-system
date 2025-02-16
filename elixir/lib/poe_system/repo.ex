defmodule PoeSystem.Repo do
  use Ecto.Repo,
    otp_app: :poe_system,
    adapter: Ecto.Adapters.Postgres
end
