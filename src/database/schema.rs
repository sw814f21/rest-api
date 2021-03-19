table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

table! {
    favorites (resturant_id, user_id){
        resturant_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    users (token_id){
        token_id -> Text,
        notification -> Integer,
    }
}