mod create;
mod schema;
mod log_entry;
mod log_auth;
mod log_posrgres;
mod i_log_reader;
mod i_log_processor;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use std::fs::File;
use std::io::Read;
use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;
use crate::create::LogDto;
use crate::i_log_processor::LogProcessor;
use crate::log_auth::LogAuth;
use crate::log_posrgres::LogPostgres;
fn main() {
    println!("Hello, world!");
    let mut file = File::open("postgresql-15-main.log").unwrap();
    let mut file2 = File::open("auth.log").unwrap();
    let mut str : String = "".to_string();
    file.read_to_string(&mut str).unwrap();
    let mut auth : String = "".to_string();
    file2.read_to_string(&mut auth).unwrap();
    let manager = ConnectionManager::<PgConnection>::new("postgres://postgres:postgres@127.0.0.1:5432/postgres");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create postgres pool.").get().unwrap();

    LogProcessor::generate(LogPostgres, &mut str, &pool);
    LogProcessor::generate(LogAuth, &mut auth, &pool)
}
