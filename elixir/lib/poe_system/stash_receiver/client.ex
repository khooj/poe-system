defmodule PoeSystem.StashReceiver.Client do
  def get_stash_data(next_stash_id, opts) do
    plug = Keyword.get(opts, :plug)
    access_token = Keyword.fetch!(opts, :access_token)

    req =
      Req.new(
        url: "https://api.pathofexile.com/public-stash-tabs",
        headers: [
          user_agent: "OAuth somepoetools/0.1.0 (contact: bladoff@gmail.com)"
        ],
        auth: {:bearer, access_token},
        retry: false,
        redirect: false,
        plug: plug,
        decode_body: false,
        params: opt(next_stash_id, id: next_stash_id)
      )

    Req.get!(req)
  end

  defp opt(nil, _), do: []
  defp opt(_, m), do: m
end
