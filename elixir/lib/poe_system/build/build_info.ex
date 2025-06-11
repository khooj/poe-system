defmodule PoeSystem.Build.BuildInfo do
  defstruct provided: nil,
            found: nil

  alias __MODULE__

  @type t :: %BuildInfo{
          provided: any(),
          found: any()
        }
end
