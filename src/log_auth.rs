use chrono::Utc;
use uuid::Uuid;
use crate::i_log_reader::ILogReader;
use crate::log_entry::LogEntry;

pub struct LogAuth;
impl ILogReader for LogAuth {
    fn convert(&self, buf: &mut String) -> Vec<LogEntry> {
        let mut logs : Vec<LogEntry> = vec![];
        let sp = buf.split("\n");
        for s in sp {
            logs.push( LogEntry {
                uuid: Uuid::new_v4(),
                date: Utc::now().naive_local(),
                severity: 1,
                message: s.to_string()
            });
            //println!("{}", s);
        }
        logs
    }
}