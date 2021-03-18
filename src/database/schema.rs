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
        city -> Nullable<Text>,
        cvr -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    restaurants,
);
