diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        confirm_password-> Varchar,
        password_hash -> Varchar,
    }
}
