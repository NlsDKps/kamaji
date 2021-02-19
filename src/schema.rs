table! {
    auth_infos (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Text,
        last_changed_date -> Date,
        last_changed_time -> Time,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        creation_date -> Date,
        creation_time -> Time,
    }
}

allow_tables_to_appear_in_same_query!(
    auth_infos,
    users,
);
