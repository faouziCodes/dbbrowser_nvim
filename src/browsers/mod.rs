use tabled::{
    builder::Builder,
    settings::{Panel, Style},
    Table,
};

pub mod postgres;
pub mod sqlite;

pub type BrowserResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct TableContents {
    pub table: String,
    pub names: Vec<String>,
    pub values: Vec<Vec<String>>,
    pub metadata: MetaDeta,
}

pub struct MetaDeta {
    query: String,
}

pub trait Browser {
    fn db_type(&self) -> &'static str;
    fn list_databases(&self) -> BrowserResult<Vec<String>>;
    fn table_contents(&self, table: &str) -> BrowserResult<TableContents>;
    fn query(&self, query: &str) -> BrowserResult<TableContents>;
}

impl Into<Table> for TableContents {
    fn into(self) -> Table {
        let mut builder = Builder::new();

        builder.push_record(self.names);
        for val in &self.values {
            builder.push_record(val);
        }

        let mut table = builder.build();
        table
            .with(Style::modern_rounded())
            .with(Panel::header(&self.table))
            .with(Panel::footer(format!("{} rows", &self.values.len())))
            .clone()
    }
}
