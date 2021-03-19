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
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    restaurants,
);
