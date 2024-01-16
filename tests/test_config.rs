use dbbrowser::config::{Config, DatabaseType};

#[test]
pub fn test_config() {
    let config: Config = toml::from_str(
        r#"
                       uri='mydb.sqlite'
                       database_type='Sqlite'
                                        "#,
    )
    .unwrap();

    assert!(config.uri == "mydb.sqlite");
    assert!(config.database_type == DatabaseType::Sqlite)
}
