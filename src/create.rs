use diesel::{PgConnection, RunQueryDsl};
use crate::log_entry::LogEntry;
use crate::schema::log_entry::dsl::log_entry;

pub struct LogDto {}

impl LogDto {
    pub fn create_list(
        conn: &PgConnection,
        log_entries: Vec<LogEntry>,
    ) -> Result<usize, diesel::result::Error> {
        Ok(diesel::insert_into(log_entry)
            .values(log_entries)
            .execute(conn)
            .unwrap())
    }
}