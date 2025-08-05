defmodule PoeSystem.PoeNinja do
  use GenServer
  require Logger
  alias Cachex
  alias PoeSystem.PoeNinja.Client

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  def get_item(name) do
    GenServer.call(__MODULE__, {:item, name})
  end

  @impl true
  def init(opts) do
    app_opts = Application.get_env(:poe_system, PoeNinja, %{disabled: false})

    if not app_opts[:disabled] do
      send(self(), :refresh_all)
    end

    {:ok, opts}
  end

  @impl true
  def handle_call({:item, name}, _from, state) do
    {:reply, Cachex.get(:poeninja, name), state}
  end

  @impl true
  def handle_info({:refresh, type}, state) do
    Logger.debug(message: "request poeninja", type: type)
    case Client.get_items("Mercenaries", type, plug: Keyword.get(state, :plug)) do
      {:ok, resp} ->
        items =
          resp.body["lines"]
          |> Enum.map(fn %{"name" => name, "chaosValue" => chaos, "divineValue" => divine} ->
            {name, %{chaos: chaos, divine: divine}}
          end)

        {:ok, true} = Cachex.put_many(:poeninja, items)
      {:error, exc} ->
        Logger.error(message: "error requesting poeninja", error: exc)
    end

    {:noreply, state}
  end

  @impl true
  def handle_info(:refresh_all, state) do
    [
      "UniqueWeapon",
      "UniqueArmour",
      "UniqueAccessory",
      "UniqueFlask",
      "UniqueJewel",
      "UniqueTincture",
      "SkillGem"
    ]
    |> Enum.each(fn type ->
      st = Keyword.get(state, :jitter_start, :timer.seconds(5))
      en = Keyword.get(state, :jitter_end, :timer.seconds(10))
      r = :rand.uniform()
      timeout = st + :math.floor((en - st) * r)
      Process.send_after(self(), {:refresh, type}, trunc(timeout))
    end)

    Process.send_after(self(), :refresh_all, Keyword.fetch!(state, :interval))
    {:noreply, state}
  end
end
