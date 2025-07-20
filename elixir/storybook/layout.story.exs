defmodule PoeSystemWeb.LayoutStory do
  use PhoenixStorybook.Story, :example
  alias PoeSystemWeb.Layouts

  def render(assigns) do
    ~H"""
      <Layouts.app inner_content="inner content" />
    """
  end
  
end
