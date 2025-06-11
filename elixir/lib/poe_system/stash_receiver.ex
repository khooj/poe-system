defmodule PoeSystem.StashReceiver do
  use Broadway
  alias PoeSystem.StashReceiver.Producer
  alias Broadway.Message

  def start_link(_opts) do
    producer = Application.fetch_env!(:poe_system, :stash_receiver_producer)

    Broadway.start_link(__MODULE__,
      name: StashReceiverImpl,
      producer: [
        module: producer,
        concurrency: 1
      ],
      processors: [
        default: [concurrency: 2]
      ]
    )
  end

  @impl true
  def handle_message(:default, %Message{data: data} = message, _) do
    %{message | metadata: %{check: true}}
  end
end
