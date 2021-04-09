table! {
    favorites (restaurant_id, token_id) {
        restaurant_id -> Integer,
        token_id -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    restaurants (id) {
        id -> Integer,
        city -> Text,
        cvr -> Text,
        latitude -> Float,
        longitude -> Float,
        pnr -> Text,
        address -> Text,
        url -> Text,
        zipcode -> Text,
        name -> Text,
    }
}

table! {
    smileyreports (id) {
        id -> Integer,
        restaurant_id -> Integer,
        rating -> Integer,
        date -> Text,
        report_id -> Text,
    }
}

table! {
    users (token_id) {
        token_id -> Text,
        notifications -> Integer,
    }
}

joinable!(favorites -> restaurants (restaurant_id));
joinable!(favorites -> users (token_id));
joinable!(smileyreports -> restaurants (restaurant_id));

allow_tables_to_appear_in_same_query!(
    favorites,
    posts,
    restaurants,
    smileyreports,
    users,
);
