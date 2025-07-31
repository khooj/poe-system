defmodule PoeSystem.PoeNinja.Client do
  @type item_type :: String.t()

  @spec get_items(String.t(), item_type(), list()) :: map()
  def get_items(league, type, opts \\ []) do
    plug = Keyword.get(opts, :plug)

    req =
      Req.new(
        url: "https://poe.ninja/api/data/itemoverview",
        headers: [
          user_agent: "OAuth somepoetools/0.1.0 (contact: bladoff@gmail.com)"
        ],
        retry: false,
        redirect: true,
        plug: plug,
        params: [league: league, type: type]
      )

    Req.get!(req)
  end
end
