use dbbrowser::browsers::Browser;
use dbbrowser::browsers::sqlite as sqlite_browser;
use std::fs::File;

pub fn test_sqlite_browser() {
    let file = File::create("test.sqlite");
    sqlite::open("test.sqlite").unwrap().execute(
        "
                                                     CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
                                                 ",
    ).unwrap();

    let databases = sqlite_browser::SqliteBrowser::new("test.sqlite").unwrap();
    assert!(databases.list_databases().unwrap().contains("users"));
}
