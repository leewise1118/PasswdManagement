use std::fs::File;

use chrono::{DateTime, Local};
use config::JSON_PATH;
use prettytable::Table;
use PM_list::passwd_mangement::{PasswdMangement, PasswdMangementList};
fn main() {
    let file = File::open(JSON_PATH).unwrap();
    let mut PML: PasswdMangementList = serde_json::from_reader(file).unwrap();

    let time = Local::now();
    let time_string: String = time.format("%Y-%m-%d %H:%M:%S").to_string();
    let PM = PasswdMangement::new(
        "WX".to_string(),
        "13349933233".to_string(),
        "123123123".to_string(),
        time_string,
    );
    PML.accounts.push(PM);
    let table: Table = PML.into();
    table.printstd();
}
