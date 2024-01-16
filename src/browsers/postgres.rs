use postgres::NoTls;

use super::{Browser, TableContents};

pub struct PostgresBrowser {
    uri: String,
}

impl PostgresBrowser {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.into() }
    }
}

impl Browser for PostgresBrowser {
    fn db_type(&self) -> &'static str {
        "postgres"
    }

    fn list_databases(&self) -> super::BrowserResult<Vec<String>> {
        let mut client = postgres::Client::connect(&self.uri, NoTls)?;
        let query = "SELECT table_name FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE';";

        let mut values: Vec<String> = Vec::new();
        for row in client.query(query, &[])? {
            let value: String = row.get(0);
            values.push(value);
        }
        client.close()?;

        Ok(values)
    }

    fn table_contents(&self, table: &str) -> super::BrowserResult<super::TableContents> {
        let mut client = postgres::Client::connect(&self.uri, NoTls)?;
        let query = format!("SELECT * FROM {table};");

        let mut table_contents = TableContents {
            names: Vec::new(),
            values: Vec::new(),
        };

        for rows in client.query(&query, &[])? {
            let mut values: Vec<String> = Vec::new();
            table_contents.values.push(values);
        }
        
        Ok(table_contents)
    }
}
