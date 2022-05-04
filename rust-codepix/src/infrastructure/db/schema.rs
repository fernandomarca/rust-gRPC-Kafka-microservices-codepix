table! {
    account (id) {
        id -> Varchar,
        owner_name -> Varchar,
        number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        bank_id -> Varchar,
        pix_keys -> Nullable<Array<Text>>,
    }
}

table! {
    bank (id) {
        id -> Varchar,
        code -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        accounts -> Nullable<Array<Text>>,
    }
}

table! {
    pixkey (id) {
        id -> Varchar,
        kind -> Varchar,
        key -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        account_id -> Varchar,
    }
}

table! {
    transaction (id) {
        id -> Varchar,
        amount -> Numeric,
        status -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        account_from_id -> Varchar,
        pix_key_id_to -> Varchar,
        pix_key_kind -> Varchar,
        pix_key_key -> Varchar,
    }
}

joinable!(account -> bank (bank_id));
joinable!(pixkey -> account (account_id));
joinable!(transaction -> account (account_from_id));
joinable!(transaction -> pixkey (pix_key_id_to));

allow_tables_to_appear_in_same_query!(
    account,
    bank,
    pixkey,
    transaction,
);
