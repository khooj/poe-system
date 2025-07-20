defmodule PoeSystemWeb.Storybook.TestLive do
  use PhoenixStorybook.Story, :example

  alias PoeSystemWeb.TestLive

  def mount(params, session, socket) do
    TestLive.mount(params, session, socket)
  end

  def render(assigns) do
    ~H"""
      <div>{TestLive.render(assigns)}</div>
    """
  end
end
