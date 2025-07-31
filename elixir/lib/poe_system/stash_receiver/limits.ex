defmodule PoeSystem.StashReceiver.Limits do
  alias PoeSystem.{RateLimitParser, RateLimit}

  def parse_and_set_ratelimits(resp) do
    %{
      "x-rate-limit-policy" => policy,
      "x-rate-limit-rules" => rules
    } = resp.headers

    policy = List.first(policy)

    rules =
      String.split(Enum.join(rules, ","), ",")
      |> Enum.map(&String.downcase/1)

    for rule <- rules do
      rules_header = "x-rate-limit-#{rule}"
      rules_state_header = "x-rate-limit-#{rule}-state"

      %{
        ^rules_header => limit,
        ^rules_state_header => limit_state
      } = resp.headers

      {
        :ok,
        limits,
        _,
        _,
        _,
        _
      } = RateLimitParser.limits(Enum.join(limit, ","))

      {
        :ok,
        limits_states,
        _,
        _,
        _,
        _
      } = RateLimitParser.limits(Enum.join(limit_state, ","))

      limits_states
      |> Enum.with_index()
      |> Enum.each(fn {ls, idx} -> set_ratelimit_state(policy, rule, idx, ls) end)

      limits
      |> Enum.with_index()
      |> Enum.map(fn {[req, sec, _], idx} ->
        {ratelimit_key(policy, rule, idx), req, sec}
      end)
    end
    |> Enum.flat_map(fn x -> x end)
  end

  defp ratelimit_key(policy, rule, idx) do
    "#{policy}_#{rule}_#{idx}"
  end

  defp set_ratelimit_state(policy, rule, idx, [req, sec, _penalty]) do
    RateLimit.set(ratelimit_key(policy, rule, idx), :timer.seconds(sec), req)
  end

  def ratelimit_allowed?(state)

  def ratelimit_allowed?(%{limits: limits}) do
    # just check limit because currect state sent by api and applied already
    for {key, req, sec} <- limits do
      count = RateLimit.get(key, :timer.seconds(sec))
      count < req
    end
    |> Enum.all?()
  end

  def ratelimit_allowed?(%{}) do
    true
  end
end
