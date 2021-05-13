table! {
    notification_history (id) {
        id -> Integer,
        subscription_id -> Integer,
        timestamp -> Text,
        data -> Text,
        title -> Text,
        body -> Text,
    }
}

table! {
    removed_restaurant (restaurant_id) {
        restaurant_id -> Integer,
        version_number -> Integer,
    }
}

table! {
    restaurant (id) {
        id -> Integer,
        smiley_restaurant_id -> Text,
        name -> Text,
        address -> Text,
        zipcode -> Text,
        city -> Text,
        cvr -> Text,
        pnr -> Text,
        latitude -> Double,
        longitude -> Double,
        version_number -> Integer,
        region -> Nullable<Text>,
        industry_code -> Text,
        industry_text -> Text,
        start_date -> Text,
        end_date -> Text,
        elite_smiley -> Text,
        niche_industry -> Text,
        url -> Text,
        ad_protection -> Text,
        company_type -> Text,
        franchise_name -> Nullable<Text>,
    }
}

table! {
    smiley_report (id) {
        id -> Integer,
        restaurant_id -> Integer,
        smiley -> Integer,
        report_id -> Text,
        date -> Text,
    }
}

table! {
    subscription (id) {
        id -> Integer,
        restaurant_id -> Integer,
        token -> Text,
    }
}

table! {
    version_history (id) {
        id -> Integer,
        timestamp -> Text,
        token -> Text,
    }
}

joinable!(notification_history -> subscription (subscription_id));
joinable!(removed_restaurant -> version_history (version_number));
joinable!(restaurant -> version_history (version_number));
joinable!(smiley_report -> restaurant (restaurant_id));
joinable!(subscription -> restaurant (restaurant_id));

allow_tables_to_appear_in_same_query!(
    notification_history,
    removed_restaurant,
    restaurant,
    smiley_report,
    subscription,
    version_history,
);
