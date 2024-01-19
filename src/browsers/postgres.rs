use super::{Browser, TableContents};
use chrono::NaiveDateTime;
use postgres::{types::Type, NoTls, Row};

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
            metadata: super::MetaDeta {
                query: query.clone(),
            },
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

            for (idx, _) in rows.columns().iter().enumerate() {
                if let Some(val) = psgst_to_string(idx, &rows) {
                    values.push(val);
                } else {
                    values.push("Null".to_string());
                };
            }

            table_contents.values.push(values);
        }

        Ok(table_contents)
    }

    fn query(&self, query: &str) -> super::BrowserResult<TableContents> {
        let mut client = postgres::Client::connect(&self.uri, NoTls)?;

        let mut table_contents = TableContents {
            table: "".into(),
            names: Vec::new(),
            values: Vec::new(),
            metadata: super::MetaDeta {
                query: query.into(),
            },
        };

        for rows in client.query(query, &[])? {
            let mut values: Vec<String> = Vec::new();

            if table_contents.names.len() == 0 {
                let cols: Vec<String> = rows
                    .columns()
                    .iter()
                    .map(|v| v.name().to_string())
                    .collect();
                table_contents.names = cols;
            }

            for (idx, _) in rows.columns().iter().enumerate() {
                if let Some(val) = psgst_to_string(idx, &rows) {
                    values.push(val);
                } else {
                    values.push("Null".to_string());
                };
            }

            table_contents.values.push(values);
        }

        Ok(table_contents)
    }
}

pub fn psgst_to_string(idx: usize, rows: &Row) -> Option<String> {
    let column = rows.columns().get(idx)?;
    let value = match column.type_() {
        &Type::INT4 => match rows.get::<_, Option<i32>>(idx) {
            Some(val) => val.to_string(),
            _ => "Null".into(),
        },
        &Type::INT8 => match rows.get::<_, Option<i64>>(idx) {
            Some(val) => val.to_string(),
            _ => "Null".into(),
        },
        &Type::TEXT | &Type::VARCHAR => match rows.get::<_, Option<String>>(idx) {
            Some(val) => val.to_string(),
            _ => "Null".into(),
        },
        &Type::FLOAT4 | &Type::FLOAT8 => match rows.get::<_, Option<f32>>(idx) {
            Some(val) => val.to_string(),
            _ => "Null".into(),
        },
        &Type::DATE => match rows.get::<_, Option<NaiveDateTime>>(idx) {
            Some(val) => val.to_string(),
            _ => "Null".into(),
        },
        &Type::TIMESTAMP => match rows.get::<_, Option<NaiveDateTime>>(idx) {
            Some(val) => val.to_string(),
            _ => "Null".into(),
        },
        _ => "".into(),
    };
    Some(value)
}
