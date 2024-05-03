// @generated automatically by Diesel CLI.

diesel::table! {
    monitoring (watcher_id, tracker_id) {
        watcher_id -> Integer,
        tracker_id -> Integer,
        tracker_name -> Text,
    }
}

diesel::table! {
    position (tracker_id, timestamp) {
        latitude -> Float,
        longitude -> Float,
        tracker_id -> Integer,
        timestamp -> BigInt,
    }
}

diesel::table! {
    tracker (id) {
        id -> Integer,
        status -> Integer,
        latitude -> Float,
        longitude -> Float,
    }
}

diesel::table! {
    watcher (id) {
        id -> Integer,
        login -> Text,
        password -> Text,
        salt -> Binary,
    }
}

diesel::joinable!(monitoring -> tracker (tracker_id));
diesel::joinable!(monitoring -> watcher (watcher_id));
diesel::joinable!(position -> tracker (tracker_id));

diesel::allow_tables_to_appear_in_same_query!(
    monitoring,
    position,
    tracker,
    watcher,
);
