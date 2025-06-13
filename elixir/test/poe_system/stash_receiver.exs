defmodule PoeSystem.StashReceiverTest do
  use ExUnit.Case
  use PoeSystemWeb.ConnCase
  alias PoeSystem.StashReceiver.Client
  alias PoeSystem.StashReceiver
  alias PoeSystem.Testdata
  alias Ecto.Multi
  alias PoeSystem.Repo
  alias PoeSystem.RateLimit

  setup do
    {:ok, opts} = StashReceiver.init(Application.fetch_env!(:poe_system, PoeSystem.StashReceiver))
    %{opts: opts}
  end

  defp put_limit_headers(conn, opts \\ []) do
    headers =
      [
        {"x-rate-limit-policy", "test-policy"},
        {"x-rate-limit-rules", "Ip,Account"},
        {"x-rate-limit-ip", "2:1:60,5:3:60"},
        {"x-rate-limit-ip-state", "1:1:60,1:3:60"},
        {"x-rate-limit-account", "10:1:60"},
        {"x-rate-limit-account-state", "1:1:60"}
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

    assert {:error, :success} =
             Repo.transaction(fn ->
               StashReceiver.handle_info(:cycle, opts)

               Repo.rollback(:success)
             end)

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

    assert_received {:ratelimited, 1}
  end

  test "rate limited by limit headers", %{opts: opts} do
    # do not use loops because we need updated state (opts)
    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers()
      |> put_resp_header("x-rate-limit-account", "1:20:60")
      |> put_resp_header("x-rate-limit-account-state", "1:20:60")
      # explicitly check that code do respect inner limits
      # so dont send 429
      |> send_resp(200, Testdata.stash_json())
    end)

    {:error, {:success, opts}} =
      Repo.transaction(fn ->
        {:noreply, opts} = StashReceiver.handle_info(:cycle, opts)

        Repo.rollback({:success, opts})
      end)

    Req.Test.stub(PoeSystem.StashReceiver, fn conn ->
      conn
      |> put_resp_content_type("application/json")
      |> put_limit_headers()
      |> put_resp_header("x-rate-limit-account", "1:20:60")
      |> put_resp_header("x-rate-limit-account-state", "2:20:60")
      |> send_resp(200, Testdata.stash_json())
    end)

    {:error, :success} =
      Repo.transaction(fn ->
        {:noreply, _} = StashReceiver.handle_info(:cycle, opts)

        Repo.rollback(:success)
      end)

    assert_received :cycle
    assert_received :ratelimited
  end
end
