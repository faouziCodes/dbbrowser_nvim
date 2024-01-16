use std::fs::File;

pub fn main() {
    let file = File::create("test.sqlite");

    sqlite::open("test.sqlite")
        .unwrap()
        .execute(
            "CREATE TABLE users (name TEXT, age INTEGER, lastname TEXT);
            INSERT INTO users VALUES ('Alice', 42, 'Hello'); 
            INSERT INTO users VALUES ('Bob', 69, 'Hi there!'); 
        ",
        )
        .unwrap();
}
