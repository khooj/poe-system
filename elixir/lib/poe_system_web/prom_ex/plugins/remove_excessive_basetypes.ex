defmodule PoeSystemWeb.PromEx.Plugins.RemoveExcessiveBasetypes do
  use PromEx.Plugin

  @impl true
  def event_metrics(_opts) do
    [
      Event.build(
        :remove_excessive_basetypes_metrics,
        [
          distribution(
            [:remove_excessive_basetypes, :done, :delta],
            event_name: [:remove_excessive_basetypes, :done],
            description: "remove excessive basetypes iteration histogram",
            measurement: :delta,
            unit: {:native, :millisecond},
            reporter_options: [
              buckets: [10, 100, 500, 1_000, 5_000, 10_000, 30_000]
            ]
          ),
        ]
      )
    ]
  end

  @impl true
  def polling_metrics(_opts), do: []

  @impl true
  def manual_metrics(_opts), do: []
end
