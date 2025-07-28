defmodule PoeSystemWeb.Components do
  use PoeSystemWeb, :html
  alias PoeSystem.Items.{Item, ItemConfig}

  attr :stat_id, :string, required: true
  attr :text, :string, required: true

  def mod_default(assigns) do
    ~H"""
      <div>{@text}</div>
    """
  end

  @rarity_color %{
    nil => "border-neutral-500",
    "normal" => "border-neutral-500",
    "magic" => "border-blue-500",
    "rare" => "border-yellow-500",
    "unique" => "border-orange-500"
  }

  attr :name, :string
  attr :basetype, :string, required: true
  attr :rarity, :string, required: true, values: ~w(normal magic rare unique)
  attr :mods, :list, required: true
  slot :mods_block
  slot :name_block

  def item(assigns) do
    rarity = Map.fetch!(@rarity_color, assigns.rarity)

    ~H"""
    <div class={["flex flex-col border divide-y", rarity]}>
      <div>
        <%= if msg = render_slot(@name_block, %{name: assigns[:name], basetype: @basetype}) do %>
          {msg}
        <% else %>
          <div class="flex flex-col">
            <p>{assigns[:name] && @name}</p>
            <p>{@basetype}</p>
          </div>
        <% end %>
      </div>
      <div>
        <div :for={mod <- @mods}>
          <%= if msg = render_slot(@mods_block, mod) do %>
            {msg}
          <% else %>
            <.mod_default {mod} />
          <% end %>
        </div>
      </div>
    </div>
    """
  end

  attr :config, ItemConfig, required: true 
  attr :item, Item, required: true

  def item_config_readonly(assigns) do
    ~H"""
      <.item 
        name={@item.name}
        basetype={@item.basetype}
        rarity={@item.rarity}
        mods={@item.info.mods}
      >
        <:name_block :let={names}>
          <div class="flex justify-between items-center">
            <div class="flex flex-col">
              <p>{names.name}</p>
              <p>{names.basetype}</p>
            </div>
            <div>
              <.label position="end" text="basetype" type="label">
                <.checkbox disabled checked={@config.basetype} />
              </.label>
              <.label position="end" text="unique" type="label">
                <.checkbox disabled checked={@config.option && @config.option == "Unique"} />
              </.label>
            </div>
          </div>
        </:name_block>
        <:mods_block :let={mod}>
          <.mod_config 
            mod={mod} 
            cfg={is_map(@config.option) && @config.option["Mods"][mod.stat_id] || nil} 
          />
        </:mods_block>
      </.item>
    """
  end

  attr :mod, :map, required: true
  attr :cfg, :any
  def mod_config(assigns)

  def mod_config(%{cfg: "Ignore"} = assigns) do
    ~H"""
      <p>{@mod.text} Ignore</p>
    """
  end

  def mod_config(%{cfg: "Exist"} = assigns) do
    ~H"""
      <p>{@mod.text} Exist</p>
    """
  end

  def mod_config(%{cfg: %{"Exact" => exact}} = assigns) do
    ~H"""
      <p>{@mod.text} Exact: {exact}</p>
    """
  end

  def mod_config(%{cfg: %{"Range" => %{"start" => start, "end" => endval}}} = assigns) do
    ~H"""
      <p>{@mod.text} Range: from {start} to {endval}</p>
    """
  end

  def mod_config(%{cfg: nil} = assigns) do
    ~H"{@mod.text}"
  end
end
