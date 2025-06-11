defmodule PoeSystem.StashReceiver.Producer do
  use GenStage
  alias Broadway.Message

  @behaviour Broadway.Producer

  def start_link(_opts) do
    GenStage.start_link(__MODULE__, [])
  end

  def init([]) do
    {:producer, []}
  end

  def handle_demand(demand, state) when demand > 0 do
    events =
      Enum.to_list(0..(demand - 1))
      |> Enum.map(fn data ->
        %Message{data: data, acknowledger: Broadway.NoopAcknowledger.init()}
      end)

    {:noreply, events, state}
  end
end
