use diesel::PgConnection;
use crate::i_log_reader::ILogReader;
use crate::LogDto;

pub struct LogProcessor {}
impl LogProcessor {
   pub fn generate<T: ILogReader>(g: T, s: &mut String,conn: &PgConnection,) {

        let log =  g.convert( s);
        LogDto::create_list(conn, log.clone()).unwrap();
        println!("{:?}", log)
    }
}