table! {
    use diesel_geometry::sql_types::Point;
    use diesel::sql_types::{Int4, Text, Timestamp, Array};

    checkins (id) {
        id -> Int4,
        gps -> Point,
        location_name -> Text,
        crowded_level -> Int4,
        missing_goods -> Array<Text>,
        user_id -> Text,
        client_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    location (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    use diesel_geometry::sql_types::Point;
    use diesel::sql_types::{Int4, Text};

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
    checkins,
    location,
    location_reports,
    products,
);
