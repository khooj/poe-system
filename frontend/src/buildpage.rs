use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::make_request::{get_build, BuildsetInfo, ItemInfo};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct SharedItemProps {
    pub info: ItemInfo,
    pub item_class: String,
    pub item_hover_class: String,
    pub item_image_width: String,
    pub image_class: String,
}

#[function_component(Item)]
pub fn item(props: &SharedItemProps) -> Html {
    let item = &props.info;
    html! {
        <div class={classes!(&props.item_class)}>
            <a href="#" alt="пример">
                <img class={classes!(&props.image_class)} src={item.image_link.clone()} width={props.item_image_width.clone()} alt="" />
                <div class={classes!("hover", &props.item_hover_class)}>
                    <div class="tooltip">
                        {&item.name} <br />
                        {&item.base_type}
                    </div>
                    <div class="info" >
                        { "Уклонение: 457" } <br />
                        { "Энерг. щит: 121" }
                    </div>
                    <div class="property">
                        { for item.mods.iter().map(|e| html!(<>{e} <br /></>)) }
                    </div>
                </div>
            </a>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ItemProps {
    pub info: ItemInfo,
}

#[function_component(Weapon1)]
pub fn weapon1(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="weapon_1" item_hover_class="weapon_1_hover" item_image_width="60" image_class="w1" />
    }
}

#[function_component(Helmet)]
pub fn helmet(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="helmet_1" item_hover_class="helmet_1_hover" item_image_width="90" image_class="w1" />
    }
}

#[function_component(Weapon2)]
pub fn weapon2(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="weapon_2" item_hover_class="weapon_2_hover" item_image_width="80" image_class="w1" />
    }
}

#[function_component(Chest)]
pub fn chest(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="chest" item_hover_class="chest_hover" item_image_width="80" image_class="w1" />
    }
}

#[function_component(Ring1)]
pub fn ring1(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="ring1" item_hover_class="ring1_hover" item_image_width="" image_class="" />
    }
}

#[function_component(Ring2)]
pub fn ring2(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="ring2" item_hover_class="ring2_hover" item_image_width="" image_class="" />
    }
}

#[function_component(Belt)]
pub fn belt(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="belt" item_hover_class="belt_hover" item_image_width="85" image_class="b1" />
    }
}

#[function_component(Amulet)]
pub fn amulet(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="amulet" item_hover_class="amulet_hover" item_image_width="" image_class="b1" />
    }
}

#[function_component(Gloves)]
pub fn gloves(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="gloves" item_hover_class="gloves_hover" item_image_width="80" image_class="w1" />
    }
}

#[function_component(Boots)]
pub fn boots(props: &ItemProps) -> Html {
    html! {
        <Item info={props.info.clone()} item_class="boots" item_hover_class="boots_hover" item_image_width="80" image_class="w1" />
    }
}

#[derive(Properties, PartialEq)]
pub struct FlasksProps {
    pub children: Children,
    pub class: String,
}

#[function_component(Flasks)]
pub fn flasks(props: &FlasksProps) -> Html {
    html! {
        <div class={classes!(&props.class)}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct FlaskProps {
    pub flask_class: String,
    pub image_link: String,
    pub hover_class: String,
    pub name: String,
    pub base_type: String,
    pub mods: Vec<String>,
}

#[function_component(Flask)]
pub fn flask(props: &FlaskProps) -> Html {
    html! {
        <div class={classes!(&props.flask_class, "flask")}>
            <a href="#" alt="пример">
                <img class={classes!("w1")} src={props.image_link.clone()} alt="" />
                <div class={classes!("hover", &props.hover_class)}>
                    <div class="tooltip_flask">
                        {&props.name} <br />
                        {&props.base_type}
                    </div>
                    <div class="info_flask" >
                        { "Уклонение: 457" } <br />
                        { "Энерг. щит: 121" }
                    </div>
                    <div class="property">
                        { for props.mods.iter().map(|e| html!(<>{e} <br /></>)) }
                    </div>
                </div>
            </a>
        </div>
    }
}

#[function_component(Flask1)]
pub fn flask1(props: &FlaskProps) -> Html {
    html! {
        <Flask ..props.clone() />
    }
}

#[function_component(Flask2)]
pub fn flask2(props: &FlaskProps) -> Html {
    html! {
        <Flask ..props.clone() />
    }
}

#[function_component(Flask3)]
pub fn flask3(props: &FlaskProps) -> Html {
    html! {
        <Flask ..props.clone() />
    }
}

#[function_component(Flask4)]
pub fn flask4(props: &FlaskProps) -> Html {
    html! {
        <Flask ..props.clone() />
    }
}

#[function_component(Flask5)]
pub fn flask5(props: &FlaskProps) -> Html {
    html! {
        <Flask ..props.clone() />
    }
}

#[derive(Properties, PartialEq)]
pub struct PriceProps {
    pub cost: i32,
}

#[function_component(Price)]
pub fn price(props: &PriceProps) -> Html {
    html! {
        <div class="price_pob">
            <div class="cost">
                <span>{ &props.cost }</span>
            </div>
            <div class="pict">
                <img class="pict" src="source/POE-chaos_orb.png" alt="chaos orb" />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BuildProps {
    name: String,
    build_class: String,
    buildset: BuildsetInfo,
}

#[function_component(BuildInventory)]
fn build_inventory(props: &BuildProps) -> Html {
    let name = "Поход греха".to_string();
    let base_type = "Шевровые ботинки".to_string();
    let mods: Vec<String> = vec!["+21 к ловкости".into(), "+23 к интеллекту".into()];

    let flask_props = FlaskProps {
        name,
        base_type,
        mods,
        ..FlaskProps::default()
    };

    html! {
            <div class={classes!(&props.build_class)}>
                <div class="name_1">
                    <div class="pobname nameblock">
                        <span>{&props.name}</span>
                    </div>
                </div>
                <Weapon1 info={props.buildset.weapon1.clone()} />
                <Helmet info={props.buildset.helmet.clone()} />
                <Weapon2 info={props.buildset.weapon2.clone()} />
                <Chest info={props.buildset.body_armour.clone()} />
                <Belt info={props.buildset.belt.clone()} />
                <Ring1 info={props.buildset.ring1.clone()} />
                <Ring2 info={props.buildset.ring2.clone()} />
                <Amulet info={props.buildset.amulet.clone()} />
                <Gloves info={props.buildset.gloves.clone()} />
                <Boots info={props.buildset.boots.clone()} />

                <Flasks class="flasks">
                    <Flask flask_class="flask_1" hover_class="flask_1_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_2" hover_class="flask_2_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_3" hover_class="flask_3_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_4" hover_class="flask_4_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_5" hover_class="flask_5_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                </Flasks>
                <Price cost=140 />
            </div>
    }
}

#[derive(Properties, PartialEq)]
struct CustomBuildInventoryProps {
    buildset: BuildsetInfo,
}

#[function_component(BuildInventoryLeft)]
fn build_inventory_left(props: &CustomBuildInventoryProps) -> Html {
    html! {
        <BuildInventory build_class="thing_start" name={"POB Build"} buildset={props.buildset.clone()} />
    }
}

#[function_component(BuildInventoryRight)]
fn build_inventory_right(props: &CustomBuildInventoryProps) -> Html {
    html! {
        <BuildInventory build_class="thing_end" name={"END build"} buildset={props.buildset.clone()} />
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(BuildPage)]
pub fn build_page(props: &Props) -> Html {
    let id = props.id.clone();
    let state = use_state(|| None);
    let req = use_async(async move { get_build(&id).await });

    {
        let req = req.clone();
        use_effect_with_deps(
            move |_| {
                req.run();
                || ()
            },
            props.id.clone(),
        );
    }

    {
        let state = state.clone();
        let req = req.clone();
        use_effect_with_deps(move |req| {
            if let Some(data) = &req.data {
                state.set(data.clone());
            }
            || ()
        }, req);
    }

    let body = {
        if let Some(data) = &*state {
            let data = data.clone();
            html! {
                <div class="thing_main">
                    <BuildInventoryLeft buildset={data.required_items} />
                    <BuildInventoryRight buildset={data.found_items} />
                </div>
            }
        } else {
            html! {
                <div><span class="info_data">{ "Build not calculated yet" }</span></div>
            }
        }
    };

    html!(
       <div class={classes!("container_main")}>
           <header>
               <div class={classes!("logo")}>
                   <img class={classes!("logoimg")} src="source/buildpage_header.png" alt="logo" />
               </div>
           </header>
           <main>
                { body }
           </main>
       </div>
    )
}
