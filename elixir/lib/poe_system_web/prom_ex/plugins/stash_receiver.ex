defmodule PoeSystemWeb.PromEx.Plugins.StashReceiver do
  use PromEx.Plugin

  @impl true
  def event_metrics(_opts) do
    [
      Event.build(
        :stash_receiver_metrics,
        [
          last_value(
            [:stash_receiver, :process_stash, :done, :delta, :last_value],
            event_name: [:stash_receiver, :process_stash, :done],
            description: "stash receiver iteration delta last value",
            measurement: :delta,
            unit: {:native, :millisecond}
          ),
          distribution(
            [:stash_receiver, :process_stash, :done, :delta],
            event_name: [:stash_receiver, :process_stash, :done],
            description: "stash receiver iteration histogram",
            measurement: :delta,
            unit: {:native, :millisecond},
            reporter_options: [
              buckets: [10, 100, 500, 1_000, 5_000, 10_000, 30_000]
            ]
          ),
          distribution(
            [:stash_receiver, :process_stash, :ratelimited, :value],
            event_name: [:stash_receiver, :process_stash, :ratelimited],
            description: "stash receiver ratelimited info",
            measurement: :value,
            unit: {:native, :millisecond},
            tags: [:retry_after, :header],
            reporter_options: [
              buckets: [10, 100, 500, 1_000, 5_000, 10_000, 30_000]
            ]
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
