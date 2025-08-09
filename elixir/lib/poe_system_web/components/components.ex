defmodule PoeSystemWeb.Components do
  use PoeSystemWeb, :html
  alias PoeSystem.Items.{Item, ItemConfig}

  attr :type, :atom, required: true
  attr :data, :any, required: true

  def mod_default(assigns) do
    ~H"""
    <div :if={assigns.data[:mods]}>
      <div :for={mod <- assigns.data.mods}>
        {mod.text}
      </div>
    </div>
    """
  end

  attr :name, :string
  attr :basetype, :string, required: true
  attr :rarity, :string, required: true, values: ~w(normal magic rare unique)
  # tuple {:atom, :list}
  attr :info, :any, required: true
  slot :mods_block
  slot :name_block

  def item(assigns) do
    assigns =
      assign(assigns, :rarity_color, %{
        nil => "border-neutral-500",
        "normal" => "border-neutral-500",
        "magic" => "border-blue-500",
        "rare" => "border-yellow-500",
        "unique" => "border-orange-500"
      })

    ~H"""
    <div class={["flex flex-col border divide-y", @rarity_color[@rarity]]}>
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
        <%= if msg = render_slot(@mods_block, %{type: elem(@info, 0), data: elem(@info, 1)}) do %>
          {msg}
        <% else %>
          <.mod_default type={elem(@info, 0)} data={elem(@info, 1)} />
        <% end %>
      </div>
    </div>
    """
  end

  attr :config, ItemConfig, required: true
  attr :item, Item, required: true

  def item_config_readonly(assigns) do
    ~H"""
    <.item name={@item.name} basetype={@item.basetype} rarity={@item.rarity} info={@item.info}>
      <:name_block :let={names}>
        <div class="flex justify-between items-center">
          <div :if={@item.category != :gems} class="flex flex-col">
            <p>{names.name}</p>
            <p>{names.basetype}</p>
          </div>
          <div :if={@item.category == :gems} class="flex flex-col p-1">
            <p>{names.name} {elem(@item.info, 1).level}/{elem(@item.info, 1).quality}%</p>
          </div>
          <div>
            <.label position="end" text="basetype" type="label">
              <.checkbox checked={@config.basetype} />
            </.label>
            <.label position="end" text="unique" type="label">
              <.checkbox checked={@config.option && @config.option == :unique} />
            </.label>
          </div>
        </div>
      </:name_block>
      <:mods_block :let={%{type: _type, data: data}}>
        <div :if={data[:mods]}>
          <div :for={mod <- data.mods}>
            <.mod_config mod={mod} option={@config.option} />
          </div>
        </div>
      </:mods_block>
    </.item>
    """
  end

  attr :item, Item, required: true

  def item_simple(assigns) do
    ~H"""
    <.item name={@item.name} basetype={@item.basetype} rarity={@item.rarity} info={@item.info}>
      <:name_block :let={names}>
        <div class="flex justify-between">
          <div :if={@item.category != :gems} class="flex flex-col">
            <p>{names[:name] && names.name}</p>
            <p>{names.basetype}</p>
          </div>
          <div :if={@item.category == :gems} class="flex flex-col p-1">
            <p>{names.name} {elem(@item.info, 1).level}/{elem(@item.info, 1).quality}%</p>
          </div>
          <div>
            <p>Price: {elem(@item.price, 0)} {elem(@item.price, 1)}</p>
          </div>
        </div>
      </:name_block>
    </.item>
    """
  end

  attr :mod, :map, required: true
  attr :option, :any, required: true
  def mod_config(assigns)

  def mod_config(%{option: {:mods, _}} = assigns) do
    ~H"""
    <div class="flex justify-between">
      <p>{@mod.text}</p>
      <p>
        <.mod_config_opt opt={Enum.find(elem(@option, 1), fn {k, _} -> k.value == @mod.stat_id end)} />
      </p>
    </div>
    """
  end

  def mod_config(%{option: :unique} = assigns) do
    ~H"""
    <p>{@mod.text}</p>
    """
  end

  attr :opt, :map, required: true

  def mod_config_opt(assigns)

  def mod_config_opt(%{opt: {_, :exist}} = assigns) do
    ~H"""
    Exist
    """
  end

  def mod_config_opt(%{opt: {_, :exact}} = assigns) do
    ~H"""
    Exact
    """
  end

  def mod_config_opt(%{opt: nil} = assigns) do
    ~H""
  end
end
