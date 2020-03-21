table! {
    checkins (id) {
        id -> Int4,
        gps_lat -> Point,
        location_name -> Text,
        crowded_level -> Int4,
        user_id -> Text,
        client_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    goods (id) {
        id -> Text,
    }
}

table! {
    missing_goods (id) {
        id -> Int4,
        checkin_id -> Nullable<Int4>,
        good_id -> Nullable<Text>,
    }
}

table! {
    supermarkets (id) {
        id -> Text,
        name -> Nullable<Text>,
        lat -> Float8,
        lon -> Float8,
        housenumber -> Nullable<Text>,
        city -> Nullable<Text>,
        country -> Nullable<Text>,
        brand -> Nullable<Text>,
    }
}

joinable!(missing_goods -> checkins (checkin_id));
joinable!(missing_goods -> goods (good_id));

allow_tables_to_appear_in_same_query!(
    checkins,
    goods,
    missing_goods,
    supermarkets,
);
