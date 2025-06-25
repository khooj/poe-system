defmodule Mix.Tasks.Poe.CheckItemIds do
  use Mix.Task
  alias PoeSystem.Repo
  alias PoeSystem.Items.Item
  import Ecto.Query
  import Mix.Ecto
  import Mix.EctoSQL

  @requirements ["app.config"]

  def run(args) do
    Application.ensure_all_started([:ecto, :postgrex])
    {:ok, pid} = Repo.start_link([])

    ids =
      IO.read(:eof)
      |> String.split()
      |> Enum.chunk_every(50)

    for ids_pack <- ids do
      if Repo.exists?(from m in Item, where: m.id in ^ids_pack) do
        Repo.all(from m in Item, where: m.id in ^ids_pack, select: m.id)
      else
        []
      end
    end
    |> Enum.flat_map(fn x -> x end)
    |> IO.inspect()
  end
end
