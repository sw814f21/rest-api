table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    favorites (resturant_id, token_id){
        resturant_id -> Integer,
        token_id -> Text,
    }
}

table! {
    users (token_id){
        token_id -> Text,
        notifications -> Integer,
    }
}