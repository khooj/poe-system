defmodule PoeSystemWeb.PromEx.Plugins.Extra do
  use PromEx.Plugin

  @impl true
  def event_metrics(_opts) do
    [
      Event.build(
        :phoenix_additional_endpoint_metrics,
        [
          last_value([:poe_system, :prom_ex, :phoenix, :endpoint, :start],
            event_name: [:phoenix, :endpoint, :start],
            description: "test metrics",
            measurement: :system_time,
            unit: {:native, :millisecond}
            # tags: [:url, :endpoint]
          ),
          last_value(
            [:poe_system, :prom_ex, :phoenix, :router_dispatch, :start],
            event_name: [:phoenix, :router_dispatch, :start],
            tags: [:route],
            unit: {:native, :millisecond},
            measurement: :system_time
          )
        ]
      )
    ]
  end

  @impl true

  def polling_metrics(_opts), do: []

  @impl true
  def manual_metrics(_opts), do: []
end
