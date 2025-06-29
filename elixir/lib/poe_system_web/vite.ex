defmodule PoeSystemWeb.Vite do
  @default_manifest_path "priv/static/manifest.json"

  def main_js do
    {_, v} = main_entry()
    "/" <> v["file"]
  end

  def main_css do
    {_, v} = main_entry()
    "/" <> List.first(v["css"])
  end

  # sobelow_skip ["Traversal.FileModule"]
  # SECURITY: constant usage
  case Application.compile_env!(:poe_system, :mode) do
    :prod ->
      defp main_entry do
        case :persistent_term.get({__MODULE__, :mains}, nil) do
          nil ->
            d = Application.app_dir(:poe_system, @default_manifest_path)
            data = Phoenix.json_library().decode!(File.read!(d))

            entry =
              data
              |> Enum.find(fn {_, v} -> v["isEntry"] end)

            :persistent_term.put({__MODULE__, :mains}, entry)
            entry

          entry ->
            entry
        end
      end

    :dev ->
      defp main_entry do
        d = Application.app_dir(:poe_system, @default_manifest_path)
        data = Phoenix.json_library().decode!(File.read!(d))

        entry =
          data
          |> Enum.find(fn {_, v} -> v["isEntry"] end)

        entry
      end

    :test ->
      defp main_entry do
        {1, %{"file" => "path", "css" => ["path"]}}
      end
  end
end
