use crate::database::Database;
use std::fs;

#[test]
fn stores_and_gets_a_value() {
    let mut db = Database::new();

    db.set(String::from("name"), String::from("Taro"));

    assert_eq!(db.get("name"), Some("Taro"));
}

#[test]
fn removes_a_value() {
    let mut db = Database::new();

    db.set(String::from("name"), String::from("Taro"));

    assert_eq!(db.remove("name"), Some(String::from("Taro")));
    assert_eq!(db.get("name"), None);
}

#[test]
fn lists_values() {
    let mut db = Database::new();

    db.set(String::from("name"), String::from("Taro"));

    let values: Vec<_> = db.list().collect();

    assert_eq!(values, vec![("name", "Taro")]);
}

#[test]
fn creates_database_file_when_loading_missing_file() {
    let path = std::env::temp_dir().join("mini-db-load-missing-test.db");

    // 前回のテスト実行でファイルが残っていても、
    // 「ファイルが無い状態」から確認できるように削除する
    let _ = fs::remove_file(&path);

    let db = Database::load(path.to_str().unwrap()).unwrap();

    assert!(path.exists());
    assert_eq!(db.list().count(), 0);

    fs::remove_file(path).unwrap();
}

#[test]
fn saves_and_loads_values() {
    // テスト専用の一時ファイルパスを作る
    let path = std::env::temp_dir().join("mini-db-save-load-test.db");

    // 前回のテストでファイルが残っている可能性があるので削除する
    let _ = std::fs::remove_file(&path);

    // 空のDBを作成する
    let mut db = Database::new();

    // register
    db.set(String::from("name"), String::from("Taro"));
    db.set(String::from("age"), String::from("20"));
    db.save(path.to_str().unwrap()).unwrap();

    // read
    let loaded_db = Database::load(path.to_str().unwrap()).unwrap();

    // check
    assert_eq!(loaded_db.get("name"), Some("Taro"));
    assert_eq!(loaded_db.get("age"), Some("20"));

    // remove
    std::fs::remove_file(path).unwrap();
}

#[test]
fn clears_all_values() {
    let mut db = Database::new();

    db.set(String::from("name"), String::from("Taro"));
    db.set(String::from("age"), String::from("20"));

    // 全件削除する
    db.clear();

    assert_eq!(db.get("name"), None);
    assert_eq!(db.get("age"), None);
}

#[test]
fn checks_if_key_exists() {
    let mut db = Database::new();
    db.set(String::from("name"), String::from("Taro"));

    assert!(db.exists("name"));
    assert!(!db.exists("age"));
}

#[test]
fn counts_values() {
    let mut db = Database::new();
    assert_eq!(db.count(), 0);

    db.set(String::from("name"), String::from("taro"));
    db.set(String::from("age"), String::from("20"));
    assert_eq!(db.count(), 2);
}

#[test]
fn saves_and_loads_value_containing_special_characters() {
    let path = std::env::temp_dir().join("mini-db-special_value_test.db");
    let _ = fs::remove_file(&path);

    let mut db = Database::new();
    let value = String::from("first line\nsecond line\\nwith\ttab");

    db.set(String::from("message"), value.clone());
    db.save(path.to_str().unwrap()).unwrap();

    let loaded_db = Database::load(path.to_str().unwrap()).unwrap();

    assert_eq!(loaded_db.get("message"), Some(value.as_str()));

    fs::remove_file(path).unwrap();
}
