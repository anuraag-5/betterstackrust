// @generated automatically by Diesel CLI.

diesel::table! {
    page_visits (id) {
        id -> Int8,
        website -> Text,
        visitor_id -> Text,
        page_path -> Text,
        referrer -> Text,
        user_agent -> Text,
        visited_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    plan (id) {
        id -> Text,
        name -> Text,
        price -> Text,
    }
}

diesel::table! {
    region (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        plan_name -> Text,
    }
}

diesel::table! {
    website_tick (id) {
        id -> Text,
        response_time_ms -> Int4,
        status -> Text,
        region_id -> Text,
        website_url -> Text,
        createdAt -> Timestamp,
    }
}

diesel::table! {
    websites (id) {
        id -> Text,
        url -> Text,
        time_added -> Timestamp,
        user_id -> Text,
        is_snippet_added -> Bool,
        about -> Text,
        plan_name -> Text,
    }
}

diesel::joinable!(website_tick -> region (region_id));
diesel::joinable!(websites -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    page_visits,
    plan,
    region,
    users,
    website_tick,
    websites,
);
