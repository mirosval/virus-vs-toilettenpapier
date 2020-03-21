#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    checkins (id) {
        id -> Int4,
        gps -> Geography,
        location_name -> Text,
        crowded_level -> Int4,
        missing_goods -> Array<Text>,
        user_id -> Text,
        client_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_geography::sql_types::*;

    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(checkins, spatial_ref_sys,);
