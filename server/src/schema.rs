table! {
    use diesel_geometry::sql_types::Point;
    use diesel::sql_types::{Text, Int4, Timestamp};

    checkins (id) {
        id -> Int4,
        gps -> Point,
        location_name -> Text,
        crowded_level -> Int4,
        user_id -> Text,
        client_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    goods (name) {
        name -> Text,
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
    use diesel::sql_types::{Text, Int4, Timestamp};

    location_reports (id) {
        id -> Text,
        coordinates -> Point,
        location_id -> Int4,
    }
}

table! {
    missing_goods (checkin_id, good_id) {
        checkin_id -> Int4,
        good_id -> Text,
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
joinable!(missing_goods -> checkins (checkin_id));
joinable!(missing_goods -> goods (good_id));

allow_tables_to_appear_in_same_query!(
    checkins,
    goods,
    location,
    location_reports,
    missing_goods,
    products,
);
