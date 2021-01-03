table! {
    accounts (id) {
        id -> Varchar,
        items -> Array<Jsonb>,
        debtors -> Array<Jsonb>,
        name_ -> Varchar,
    }
}
