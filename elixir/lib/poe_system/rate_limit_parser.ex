defmodule PoeSystem.RateLimitParser do
  import NimbleParsec

  limit =
    integer(min: 1)
    |> ignore(string(":"))
    |> integer(min: 1)
    |> ignore(string(":"))
    |> integer(min: 1)
    |> optional(ignore(string(",")))

  defparsec(:limits, times(wrap(limit), min: 1))
end
