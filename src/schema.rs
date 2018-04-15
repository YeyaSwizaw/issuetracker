table! {
    issues (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        created -> Date,
    }
}