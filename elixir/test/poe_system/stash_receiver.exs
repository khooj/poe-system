defmodule PoeSystem.StashReceiverTest do
  use ExUnit.Case
  use PoeSystemWeb.ConnCase
  alias PoeSystem.{StashReceiver, Testdata, Repo, LatestStash}
  alias PoeSystem.Items.Item

  setup do
    {:ok, opts} = StashReceiver.init(Application.fetch_env!(:poe_system, PoeSystem.StashReceiver))
    %{opts: opts}
  end

  defp put_limit_headers(conn, opts \\ []) do
    headers =
      [
        {"x-rate-limit-policy", "test-policy"},
        {"x-rate-limit-rules", "Ip"},
        {"x-rate-limit-ip", "10:1:60,20:3:60"},
        {"x-rate-limit-ip-state", "1:1:60,1:3:60"}
      ] ++
        if Keyword.get(opts, :with_retry, false) do
          [{"retry-after", "1"}]
        else
          []
        end

    Enum.reduce(headers, conn, fn {k, v}, conn ->
      put_resp_header(conn, k, v)
    end)
  end

  test "process stash data from api", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers()
      |> send_resp(200, Testdata.stash_json())
    end)

    assert {:noreply, _} = StashReceiver.handle_info(:cycle, opts)
    assert_receive :cycle
    assert Repo.exists?(Item)
  end

  test "league filter", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers()
      |> send_resp(200, Testdata.stash_json())
    end)

    opts = Map.put(opts, :league, ["league not exist"])
    assert {:noreply, _} = StashReceiver.handle_info(:cycle, opts)
    assert not Repo.exists?(Item)
  end

  test "request api w/ latest stash id", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers()
      |> send_resp(200, Testdata.stash_json())
    end)

    Repo.insert!(%LatestStash{id: "test"})

    assert {:noreply, _} = StashReceiver.handle_info(:cycle, opts)
    assert_receive :cycle
  end

  test "rate limited by 429", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers(with_retry: true)
      |> send_resp(429, "Timeout")
    end)

    {:noreply, _} = StashReceiver.handle_info(:cycle, opts)

    interval = opts.interval
    assert_received {:ratelimited, ^interval}
  end

  test "rate limited by limit headers", %{opts: opts} do
    # do not use loops because we need updated state (opts)
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      # hand-crafted headers so we does not interfere with global
      # rate limits set by other tests
      |> put_resp_header("x-rate-limit-policy", "test-policy")
      |> put_resp_header("x-rate-limit-rules", "Account")
      |> put_resp_header("x-rate-limit-account", "1:20:60")
      |> put_resp_header("x-rate-limit-account-state", "1:20:60")
      |> send_resp(200, Testdata.stash_json())
    end)

    # using explicit transaction to reset implicit test transaction
    {:error, {:success, opts}} =
      Repo.transaction(fn ->
        {:noreply, opts} = StashReceiver.handle_info(:cycle, opts)
        Repo.rollback({:success, opts})
      end)

    Req.Test.stub(PoeSystem.StashReceiver, fn _ ->
      raise "Should not be called"
    end)

    {:noreply, _} = StashReceiver.handle_info(:cycle, opts)

    assert_receive :cycle
    assert_receive :ratelimited
  end

  test "500", %{opts: opts} do
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers()
      |> send_resp(500, "Internal server error")
    end)

    {:noreply, _} = StashReceiver.handle_info(:cycle, opts)

    long_interval = opts.long_interval
    assert_received {:ratelimited, ^long_interval}
  end
end
