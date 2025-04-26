defmodule PoeSystem.BuildInfo.BuildData do
  alias PoeSystem.BuildInfo.BuildItemsWithConfig

  @type t() ::
          %{
            :provided => BuildItemsWithConfig.t(),
            :found => any()
          }
end
