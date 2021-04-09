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

joinable!(notification_history -> subscription (subscription_id));
joinable!(smiley_report -> restaurant (restaurant_id));
joinable!(subscription -> restaurant (restaurant_id));

allow_tables_to_appear_in_same_query!(
    notification_history,
    restaurant,
    smiley_report,
    subscription,
);
