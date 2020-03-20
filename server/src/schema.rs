table! {
    location (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geometry::sql_types::*;
    location_reports (id) {
        id -> Text,
        coordinates -> Point,
        location_id -> Int4,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Text,
        reference -> Text,
    }
}

joinable!(location_reports -> location (location_id));

allow_tables_to_appear_in_same_query!(
    location,
    location_reports,
    products,
);
