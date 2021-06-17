table! {
    build_info (id) {
        id -> Text,
        pob_url -> Text,
        itemset -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    build_info,
);
