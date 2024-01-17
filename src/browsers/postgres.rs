use super::{Browser, TableContents};
use chrono::{DateTime, Utc, NaiveDateTime};
use postgres::{types::Type, NoTls};

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
            table: table.into(),
            names: Vec::new(),
            values: Vec::new(),
        };

        for rows in client.query(&query, &[])? {
            let mut values: Vec<String> = Vec::new();

            if table_contents.names.len() == 0 {
                let cols: Vec<String> = rows
                    .columns()
                    .iter()
                    .map(|v| v.name().to_string())
                    .collect();
                table_contents.names = cols;
            }

            for (idx, column) in rows.columns().iter().enumerate() {
                let value = match column.type_() {
                    &Type::INT4 => rows.get::<_, i32>(idx).to_string(),
                    &Type::INT8 => rows.get::<_, i64>(idx).to_string(),
                    &Type::TEXT | &Type::VARCHAR => rows.get::<_, String>(idx),
                    &Type::FLOAT4 | &Type::FLOAT8 => rows.get::<_, f32>(idx).to_string(),
                    &Type::DATE => rows.get::<_, DateTime<Utc>>(idx).to_string(),
                    &Type::TIMESTAMP => rows.get::<_, NaiveDateTime>(idx).to_string(),
                    _ => "".into(),
                };
                values.push(value);
            }

            table_contents.values.push(values);
        }

        Ok(table_contents)
    }
}
