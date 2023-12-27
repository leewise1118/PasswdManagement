pub mod passwd_mangement;
use clap::Parser;
use config::JSON_PATH;
use passwd_mangement::PasswdMangementList;
use prettytable::Table;
use std::fs::File;

fn main() {
    let file = File::open(JSON_PATH).unwrap();
    let deserialized: PasswdMangementList = serde_json::from_reader(file).unwrap();
    let table: Table = deserialized.into();
    table.printstd();
}
