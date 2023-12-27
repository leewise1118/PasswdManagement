use prettytable::{row, Row, Table};
use serde::{Deserialize, Serialize};

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswdMangement {
    tag: String,
    username: String,
    password: String,
    update_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswdMangementList {
   pub  accounts: Vec<PasswdMangement>,
}

impl Into<Row> for PasswdMangement {
    fn into(self) -> Row {
        row![self.tag, self.username, self.password, self.update_time]
    }
}

impl Into<Table> for PasswdMangementList {
    fn into(self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Tag", "Username", "Password", "Update Time"]);
        for account in self.accounts {
            table.add_row(account.into());
        }
        table
    }
}

impl PasswdMangement {
    pub fn new(tag: String, username: String, password: String, update_time: String) -> Self {
        Self {
            tag,
            username,
            password,
            update_time,
        }
    }
}
// impl PasswdMangementList {
//     pub fn push(mut self, PM: PasswdMangement) -> Self {
//         self.accounts.push(PM);
//         self
//     }
// }
