use crate::log_entry::LogEntry;

pub trait ILogReader {
    fn convert(&self, buf: &mut String) -> Vec<LogEntry>;
}