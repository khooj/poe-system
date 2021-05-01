table! {
    extended (item_id) {
        item_id -> Nullable<Text>,
        category -> Text,
        prefixes -> Nullable<Integer>,
        suffixes -> Nullable<Integer>,
    }
}

table! {
    hybrids (id, item_id) {
        id -> Nullable<Text>,
        item_id -> Text,
        is_vaal_gem -> Nullable<Bool>,
        base_type_name -> Text,
        sec_descr_text -> Nullable<Text>,
    }
}

table! {
    incubated_item (item_id, name) {
        item_id -> Text,
        name -> Text,
        level -> Integer,
        progress -> Integer,
        total -> Integer,
    }
}

table! {
    influences (item_id) {
        item_id -> Nullable<Text>,
        warlord -> Nullable<Bool>,
        crusader -> Nullable<Bool>,
        redeemer -> Nullable<Bool>,
        hunter -> Nullable<Bool>,
    }
}

table! {
    items (id) {
        id -> Text,
        base_type -> Text,
        account_id -> Text,
        stash_id -> Text,
        league -> Nullable<Text>,
        name -> Text,
        item_lvl -> Nullable<Integer>,
        identified -> Bool,
        inventory_id -> Nullable<Text>,
        type_line -> Text,
        abyss_jewel -> Nullable<Bool>,
        corrupted -> Nullable<Bool>,
        duplicated -> Nullable<Bool>,
        elder -> Nullable<Bool>,
        frame_type -> Nullable<Integer>,
        h -> Integer,
        w -> Integer,
        x_coordinate -> Nullable<Integer>,
        y_coordinate -> Nullable<Integer>,
        is_relic -> Nullable<Bool>,
        note -> Nullable<Text>,
        shaper -> Nullable<Bool>,
        stack_size -> Nullable<Integer>,
        max_stack_size -> Nullable<Integer>,
        support -> Nullable<Bool>,
        talisman_tier -> Nullable<Integer>,
        verified -> Bool,
        icon -> Text,
        delve -> Nullable<Bool>,
        fractured -> Nullable<Bool>,
        synthesised -> Nullable<Bool>,
        split -> Nullable<Bool>,
        sec_descr_text -> Nullable<Text>,
        veiled -> Nullable<Bool>,
        descr_text -> Nullable<Text>,
        prophecy_text -> Nullable<Text>,
        replica -> Nullable<Bool>,
        socket -> Nullable<Integer>,
        colour -> Nullable<Text>,
    }
}

table! {
    latest_stash_id (id) {
        id -> Nullable<Text>,
    }
}

table! {
    mods (item_id, mod_) {
        item_id -> Text,
        #[sql_name = "type"]
        type_ -> Integer,
        #[sql_name = "mod"]
        mod_ -> Text,
    }
}

table! {
    properties (item_id, name) {
        item_id -> Text,
        property_type -> Integer,
        name -> Text,
        value_type -> Integer,
        value -> Integer,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        progress -> Nullable<Float>,
        suffix -> Nullable<Text>,
    }
}

table! {
    socketed_items (item_id, socketed_item_id) {
        item_id -> Text,
        socketed_item_id -> Text,
    }
}

table! {
    sockets (item_id, s_group) {
        item_id -> Text,
        s_group -> Integer,
        attr -> Nullable<Text>,
        s_colour -> Nullable<Text>,
    }
}

table! {
    subcategories (item_id, subcategory) {
        item_id -> Text,
        subcategory -> Text,
    }
}

table! {
    ultimatum_mods (item_id, type_) {
        item_id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        tier -> Integer,
    }
}

joinable!(extended -> items (item_id));
joinable!(hybrids -> items (item_id));
joinable!(incubated_item -> items (item_id));
joinable!(influences -> items (item_id));
joinable!(mods -> items (item_id));
joinable!(properties -> items (item_id));
joinable!(socketed_items -> items (item_id));
joinable!(sockets -> items (item_id));
joinable!(subcategories -> items (item_id));
joinable!(ultimatum_mods -> items (item_id));

allow_tables_to_appear_in_same_query!(
    extended,
    hybrids,
    incubated_item,
    influences,
    items,
    latest_stash_id,
    mods,
    properties,
    socketed_items,
    sockets,
    subcategories,
    ultimatum_mods,
);
