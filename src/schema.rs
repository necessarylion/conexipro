// @generated automatically by Diesel CLI.

diesel::table! {
    user_infos (id) {
        id -> Unsigned<Bigint>,
        user_id -> Integer,
        #[max_length = 255]
        info_key -> Varchar,
        #[max_length = 255]
        info_value -> Nullable<Varchar>,
        #[max_length = 100]
        info_type -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        middle_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        other_name -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        avatar -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user_infos,
    users,
);
