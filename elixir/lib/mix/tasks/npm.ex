defmodule Mix.Tasks.Npm do
  use Mix.Task

  def run(_) do
    Mix.shell().cmd(~s(npm run --prefix assets build))
  end
end
