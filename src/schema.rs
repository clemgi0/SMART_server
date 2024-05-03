// @generated automatically by Diesel CLI.

diesel::table! {
    positions_history (protected_id, timestamp) {
        latitude -> Float,
        longitude -> Float,
        protected_id -> Integer,
        timestamp -> BigInt,
    }
}

diesel::table! {
    protected (id) {
        id -> Integer,
        status -> Integer,
    }
}

diesel::table! {
    protection (protected_id, protector_id) {
        protected_id -> Integer,
        protector_id -> Integer,
        protected_name -> Text,
    }
}

diesel::table! {
    protector (id) {
        id -> Integer,
        login -> Text,
        password -> Text,
        salt -> Binary,
    }
}

diesel::joinable!(positions_history -> protected (protected_id));
diesel::joinable!(protection -> protected (protected_id));
diesel::joinable!(protection -> protector (protector_id));

diesel::allow_tables_to_appear_in_same_query!(
    positions_history,
    protected,
    protection,
    protector,
);
