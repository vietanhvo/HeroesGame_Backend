table! {
    #[allow(non_snake_case)]
    User (id) {
        id -> Integer,
        first_name -> Varchar,
        surname -> Varchar,
        email -> Varchar,
        date_of_birth -> Date,
        gender -> Char,
        password -> Varchar,
    }
}
