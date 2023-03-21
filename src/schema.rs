table! {
    log_entry (uuid) {
        uuid -> Uuid,
        date -> Timestamp,
        severity -> Int4,
        message -> Varchar,
    }
}