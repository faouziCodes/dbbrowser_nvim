use tabled::{builder::Builder, settings::Style, Table};

pub mod sqlite;
pub mod postgres;

pub type BrowserResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct TableContents {
    pub names: Vec<String>,
    pub values: Vec<Vec<String>>,
}

pub trait Browser {
    fn db_type(&self) -> &'static str;
    fn list_databases(&self) -> BrowserResult<Vec<String>>;
    fn table_contents(&self, table: &str) -> BrowserResult<TableContents>;
}

impl Into<Table> for TableContents {
    fn into(self) -> Table {
        let mut builder = Builder::new();


        builder.push_record(self.names);
        for val in self.values {
            builder.push_record(val);
        }

        let mut table = builder.build();
        table.with(Style::modern_rounded()).clone()
    }
}
