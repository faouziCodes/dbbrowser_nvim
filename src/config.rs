use std::fs::read_to_string;

use nvim_oxi as oxi;
use serde::Deserialize;

use crate::browsers::{sqlite, Browser, postgres};

#[derive(Deserialize, PartialEq, Eq)]
pub enum DatabaseType {
    Sqlite,
    Postgres
}

#[derive(Deserialize)]
pub struct Config {
    pub uri: String,
    pub database_type: DatabaseType,
}

impl Config {
    pub fn get_browser(&self) -> Box<dyn Browser> {
        match self.database_type {
            DatabaseType::Sqlite => Box::new(sqlite::SqliteBrowser::new(&self.uri)),
            DatabaseType::Postgres => Box::new(postgres::PostgresBrowser::new(&self.uri)),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = {
        let Ok(config) = read_to_string(".browser.toml") else {
            oxi::api::err_writeln("Could not read .browser.toml");
            return Config {
                uri: "".into(),
                database_type: DatabaseType::Sqlite
            }
        };

        match toml::from_str(&config) {
            Ok(config) => config,
            Err(err) => {
                oxi::api::err_write(&format!("Browser: {err}"));
                return Config {
                    uri: "".into(),
                    database_type: DatabaseType::Sqlite
                }
            }
        }
    };
}
