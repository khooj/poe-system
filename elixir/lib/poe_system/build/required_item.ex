defmodule PoeSystem.Build.RequiredItem do
  defstruct valid?: true
  alias __MODULE__

  @type t :: %RequiredItem{}
  @type item_type :: :accessory | :gem | :armor | :weapon | :jewel | :flask
end
