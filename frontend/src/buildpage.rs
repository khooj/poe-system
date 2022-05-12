use yew::{prelude::*, props};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct SharedItemProps {
    pub image_link: String,
    pub name: String,
    pub base_type: String,
    pub mods: Vec<String>,
    pub item_class: String,
    pub item_hover_class: String,
    pub item_image_width: String,
    pub image_class: String,
}

#[function_component(Item)]
pub fn item(props: &SharedItemProps) -> Html {
    html! {
        <div class={classes!(&props.item_class)}>
            <a href="#" alt="пример">
                <img class={classes!(&props.image_class)} src={props.image_link.clone()} width={props.item_image_width.clone()} alt="" />
                <div class={classes!("hover", &props.item_hover_class)}>
                    <div class="tooltip">
                        {&props.name} <br />
                        {&props.base_type}
                    </div>
                    <div class="info" >
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

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ItemProps {
    pub image_link: String,
    pub name: String,
    pub base_type: String,
    pub mods: Vec<String>,
    pub item_class: String,
    pub item_hover_class: String,
}

impl Into<SharedItemProps> for ItemProps {
    fn into(self) -> SharedItemProps {
        let ItemProps {
            image_link,
            name,
            base_type,
            mods,
            item_class,
            item_hover_class,
        } = self;
        SharedItemProps {
            image_link,
            name,
            base_type,
            mods,
            item_class,
            item_hover_class,
            ..SharedItemProps::default()
        }
    }
}

#[function_component(Weapon1)]
pub fn weapon1(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="60" image_class="w1" ..props />
    }
}

#[function_component(Helmet)]
pub fn helmet(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="90" image_class="w1" ..props />
    }
}

#[function_component(Weapon2)]
pub fn weapon2(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="80" image_class="w1" ..props />
    }
}

#[function_component(Chest)]
pub fn chest(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="80" image_class="w1" ..props />
    }
}

#[function_component(Ring1)]
pub fn ring1(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="" ..props />
    }
}

#[function_component(Ring2)]
pub fn ring2(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="" ..props />
    }
}

#[function_component(Belt)]
pub fn belt(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="85" image_class="b1" ..props />
    }
}

#[function_component(Amulet)]
pub fn amulet(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="" image_class="b1" ..props />
    }
}

#[function_component(Gloves)]
pub fn gloves(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="80" image_class="w1" ..props />
    }
}

#[function_component(Boots)]
pub fn boots(props: &ItemProps) -> Html {
    let props: SharedItemProps = props.clone().into();
    html! {
        <Item item_image_width="80" image_class="w1" ..props />
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
pub struct BuildProps {
    pub name: String,
}

#[function_component(BuildInventoryLeft)]
pub fn build_inventory_left(props: &BuildProps) -> Html {
    let name = "Поход греха".to_string();
    let base_type = "Шевровые ботинки".to_string();
    let mods: Vec<String> = vec![
        "+21 к ловкости".into(),
        "+23 к интеллекту".into(),
    ];

    let item_props = ItemProps {
        name: name.clone(), 
        base_type: base_type.clone(), 
        mods: mods.clone(), 
        ..ItemProps::default()
    };

    let flask_props = FlaskProps {
        name,
        base_type,
        mods,
        ..FlaskProps::default()
    };

    html! {
            <div class="thing_start">
                <div class="name_1">
                    <div class="pobname nameblock">
                        <span>{&props.name}</span>
                    </div>
                </div>
                <Weapon1 item_class="weapon_s_1" item_hover_class="weapon_s_1_hover" image_link="source/pic/w1.png" ..item_props.clone() />
                <Helmet item_class="helmet_s_1" item_hover_class="helmet_s_1_hover" image_link="source/pic/BoneHelm.png" ..item_props.clone() />
                <Weapon2 item_class="weapon_s_2" item_hover_class="weapon_s_2_hover" image_link="source/pic/w2.png" ..item_props.clone() />
                <Chest item_class="chest_s" item_hover_class="chest_s_hover" image_link="source/pic/chestr.png" ..item_props.clone() />
                <Belt item_class="belt_s" item_hover_class="belt_s_hover" image_link="source/pic/AbyssBelt.png" ..item_props.clone() />
                <Ring1 item_class="ring1_s" item_hover_class="ring1_s_hover" image_link="source/pic/ring1.png" ..item_props.clone() />
                <Ring2 item_class="ring2_s" item_hover_class="ring2_s_hover" image_link="source/pic/Ring5.png" ..item_props.clone() />
                <Amulet item_class="amulet_s" item_hover_class="amulet_s_hover" image_link="source/pic/TurquoiseAmulet.png" ..item_props.clone() />
                <Gloves item_class="gloves_s" item_hover_class="gloves_s_hover" image_link="source/pic/glove.png" ..item_props.clone() />
                <Boots item_class="boots_s" item_hover_class="boots_s_hover" image_link="source/pic/boot.png" ..item_props.clone() />
                <Flasks class="flask_start">
                    <Flask flask_class="flask_1" hover_class="flask_1_s_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_2" hover_class="flask_2_s_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_3" hover_class="flask_3_s_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_4" hover_class="flask_4_s_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_5" hover_class="flask_5_s_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                </Flasks>
                <Price cost=140 />
            </div>
    }
}

#[function_component(BuildInventoryRight)]
pub fn build_inventory_right(props: &BuildProps) -> Html {
    let name = "Поход греха".to_string();
    let base_type = "Шевровые ботинки".to_string();
    let mods: Vec<String> = vec![
        "+21 к ловкости".into(),
        "+23 к интеллекту".into(),
    ];

    let item_props = ItemProps {
        name: name.clone(), 
        base_type: base_type.clone(), 
        mods: mods.clone(), 
        ..ItemProps::default()
    };

    let flask_props = FlaskProps {
        name,
        base_type,
        mods,
        ..FlaskProps::default()
    };

    html! {
            <div class="thing_end">
                <div class="name_1">
                    <div class="pobname nameblock">
                        <span>{&props.name}</span>
                    </div>
                </div>
                <Weapon1 item_class="weapon_e_1" item_hover_class="weapon_e_1_hover" image_link="source/pic/w1.png" ..item_props.clone() />
                <Helmet item_class="helmet_e_1" item_hover_class="helmet_e_1_hover" image_link="source/pic/BoneHelm.png" ..item_props.clone() />
                <Weapon2 item_class="weapon_e_2" item_hover_class="weapon_e_2_hover" image_link="source/pic/w2.png" ..item_props.clone() />
                <Chest item_class="chest_e" item_hover_class="chest_e_hover" image_link="source/pic/chestr.png" ..item_props.clone() />
                <Belt item_class="belt_e" item_hover_class="belt_e_hover" image_link="source/pic/AbyssBelt.png" ..item_props.clone() />
                <Ring1 item_class="ring1_e" item_hover_class="ring1_e_hover" image_link="source/pic/ring1.png" ..item_props.clone() />
                <Ring2 item_class="ring2_e" item_hover_class="ring2_e_hover" image_link="source/pic/Ring5.png" ..item_props.clone() />
                <Amulet item_class="amulet_e" item_hover_class="amulet_e_hover" image_link="source/pic/TurquoiseAmulet.png" ..item_props.clone() />
                <Gloves item_class="gloves_e" item_hover_class="gloves_e_hover" image_link="source/pic/glove.png" ..item_props.clone() />
                <Boots item_class="boots_e" item_hover_class="boots_e_hover" image_link="source/pic/boot.png" ..item_props.clone() />
                <Flasks class="flask_end">
                    <Flask flask_class="flask_1_e" hover_class="flask_1_e_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_2_e" hover_class="flask_2_e_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_3_e" hover_class="flask_3_e_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_4_e" hover_class="flask_4_e_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                    <Flask flask_class="flask_5_e" hover_class="flask_5_e_hover" image_link="source/pic/lifeflask12.png" ..flask_props.clone() />
                </Flasks>
            </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(BuildPage)]
pub fn build_page(props: &Props) -> Html {
    html!(
       <div class={classes!("container_main")}>
           <header>
               <div class={classes!("logo")}>
                   <img class={classes!("logoimg")} src="source/buildpage_header.png" alt="logo" />
               </div>
           </header>
           <main>
                <div class="thing_main">
                    <BuildInventoryLeft name={"POB Build".to_string()} />
                    <BuildInventoryRight name={"END build".to_string()} />
               </div>
           </main>
       </div>
    )
}
