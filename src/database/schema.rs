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
        smiley_restaurant_id -> Integer,
        name -> Text,
        address -> Text,
        zipcode -> Text,
        city -> Text,
        cvr -> Text,
        pnr -> Text,
        latitude -> Float,
        longitude -> Float,
        version_number -> Integer,
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
