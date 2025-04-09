defmodule Routes do
  @moduledoc """
  Routes makes your Phoenix routes available in JavaScript.
  """

  defmacro __using__(_opts \\ []) do
    quote do
      @after_compile {Routes, :after_compile}
    end
  end

  def after_compile(env, _bytecode) do
    Routes.Processor.process_routes(env.module)
  end
end
