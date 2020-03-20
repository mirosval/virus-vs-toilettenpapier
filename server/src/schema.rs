table! {
    location (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Text,
        reference -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    location,
    products,
);
