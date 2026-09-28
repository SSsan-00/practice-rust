use crate::database::Database;
use crate::storage::{FileStorage, Storage};
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

    let mut storage = FileStorage::new(path.clone());

    let db = Database::load(&mut storage).unwrap();

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
    let mut storage: Box<dyn Storage> = Box::new(FileStorage::new(path.clone()));
    let mut db = Database::new();

    // 値を登録
    db.set(String::from("name"), String::from("Taro"));
    db.set(String::from("age"), String::from("20"));
    db.save(storage.as_mut()).unwrap();

    // ファイルから読み込む
    let loaded_db = Database::load(storage.as_mut()).unwrap();

    // 読み込んだ値を確認
    assert_eq!(loaded_db.get("name"), Some("Taro"));
    assert_eq!(loaded_db.get("age"), Some("20"));

    // テスト用ファイルを削除
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

    let mut storage = FileStorage::new(path.clone());
    let mut db = Database::new();
    let value = String::from("first line\nsecond line\\nwith\ttab");

    db.set(String::from("message"), value.clone());
    db.save(&mut storage).unwrap();

    let loaded_db = Database::load(&mut storage).unwrap();

    assert_eq!(loaded_db.get("message"), Some(value.as_str()));

    fs::remove_file(path).unwrap();
}

#[test]
fn loads_legacy_key_value_format() {
    // 旧形式のDBファイルを用意する
    let path = std::env::temp_dir().join("mini-db-legacy-format-test.db");
    fs::write(&path, "name=Taro\nage=20\n").unwrap();

    let mut storage = FileStorage::new(path.clone());
    let db = Database::load(&mut storage).unwrap();

    assert_eq!(db.get("name"), Some("Taro"));
    assert_eq!(db.get("age"), Some("20"));

    fs::remove_file(path).unwrap();
}
