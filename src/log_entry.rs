use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::schema::log_entry;
#[derive(
Clone,
Serialize,
Deserialize,
Debug,
PartialEq,
Queryable,
Insertable,
AsChangeset,
Identifiable,
QueryableByName,
Default,
)]
#[primary_key(uuid)]
#[table_name = "log_entry"]
pub struct LogEntry {
    pub uuid : Uuid,
    pub date: NaiveDateTime,
    pub severity: i32,
    pub message: String,
}
