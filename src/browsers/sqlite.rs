use super::{Browser, BrowserResult, TableContents};

pub struct SqliteBrowser {
    uri: String,
}

impl SqliteBrowser {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.into() }
    }
}

impl Browser for SqliteBrowser {
    fn db_type(&self) -> &'static str {
        "sqlite"
    }

    fn list_databases(&self) -> BrowserResult<Vec<String>> {
        let conn = sqlite::open(&self.uri)?;
        let query =
            "SELECT name FROM sqlite_schema WHERE type ='table' AND name NOT LIKE 'sqlite_%';";
        let mut vals = Vec::new();

        conn.iterate(query, |pairs| {
            for &(_, value) in pairs.iter() {
                match value {
                    Some(val) => vals.push(val.to_string()),
                    None => continue,
                }
            }
            true
        })?;

        Ok(vals)
    }

    fn table_contents(&self, table: &str) -> BrowserResult<super::TableContents> {
        let conn = sqlite::open(&self.uri)?;
        let query = format!("SELECT * FROM {table};");

        let mut table_contents = TableContents {
            table: table.into(),
            names: Vec::new(),
            values: Vec::new(),
        };

        conn.iterate(query, |pairs| {
            let mut values = Vec::new();
            for &(name, value) in pairs.iter() {
                if !table_contents.names.contains(&name.into()) {
                    table_contents.names.push(name.to_string());
                }

                match value {
                    Some(val) => values.push(val.to_string()),
                    None => continue,
                }
            }
            table_contents.values.push(values);
            true
        })?;

        Ok(table_contents)
    }
}
