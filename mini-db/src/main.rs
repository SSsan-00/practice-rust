mod database;

use database::Database;
use std::io::{self, Write};

enum Command {
    Exit,
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    List,
    Save,
    Clear,
    Exists { key: String },
    Count,
    Help,
    Unknown(String),
}

fn parse_command(input: &str) -> Command {
    let words: Vec<&str> = input.split_whitespace().collect();

    // words.as_slice(): &[&str]
    match words.as_slice() {
        [".exit"] => Command::Exit,
        // key以降の文字列をまとめてvaluesとしてまとめる[word1, word2, word3・・・]
        ["set", key, values @ ..] if !values.is_empty() => Command::Set {
            key: key.to_string(),
            // 空白繋ぎで配列要素(values)を展開する[word1 word2 word3 ・・・]
            value: values.join(" "),
        },
        ["get", key] => Command::Get {
            key: key.to_string(),
        },
        ["remove", key] => Command::Remove {
            key: key.to_string(),
        },
        ["list"] => Command::List,
        ["save"] => Command::Save,
        ["clear"] => Command::Clear,
        ["exists", key] => Command::Exists {
            key: key.to_string(),
        },
        ["count"] => Command::Count,
        ["help"] => Command::Help,
        _ => Command::Unknown(input.to_string()),
    }
}

/// 使用できるコマンドとその用途を表示します。
fn print_help() {
    println!(
        "Command:
                set <key> <value> データを保存する
                get <key>         データを取得する
                remove <key>      データを削除する
                list              全データを表示する
            　　save              ファイルに保存する
            　　clear             全データを削除する
            　　exists <key>      キーの有無を確認する
            　　count             データ件数を表示する
            　　help              この一覧を表示する
            　　.exit             終了する"
    );
}

fn main() -> io::Result<()> {
    // 入力を受け取る
    let mut input = String::new();

    // DBを作成する
    let mut db = Database::load("mini.db")?;
    'command_loop: loop {
        // 前回の入力を空にする
        input.clear();

        print!("db > ");

        // flushで即座に表示を端末に送る
        io::stdout().flush()?;

        // Enterまでの1行をinputに読み込む
        // 0はEOF(入力の終端)なので終了する
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        // 入力で受け取った文字列をCommandに変換する
        let command = parse_command(input.trim());

        match command {
            Command::Exit => {
                // 正しい入力が行われるまで処理を繰り返す
                loop {
                    // 終了前に保存するか確認する
                    print!("Save before exit? (y/n): ");
                    io::stdout().flush()?;

                    // 以前のコマンド入力をクリアする
                    input.clear();

                    if io::stdin().read_line(&mut input)? == 0 {
                        break 'command_loop;
                    }

                    match input.trim() {
                        "y" | "Y" => {
                            db.save("mini.db")?;
                            println!("saved");
                            break 'command_loop;
                        }
                        "n" | "N" => {
                            // 保存せずにそのまま終了する
                            break 'command_loop;
                        }
                        _ => {
                            // リトライ
                            println!("Please enter y or n.");
                        }
                    }
                }
            }
            Command::Set { key, value } => {
                db.set(key, value);
                println!("OK")
            }
            Command::Get { key } => match db.get(&key) {
                Some(value) => println!("{value}"),
                None => println!("(nil)"),
            },
            Command::Remove { key } => match db.remove(&key) {
                Some(_) => println!("OK"),
                None => println!("(nil)"),
            },
            Command::List => {
                for (key, value) in db.list() {
                    println!("{key} = {value}");
                }
            }
            Command::Save => {
                db.save("mini.db")?;
                println!("saved");
            }
            Command::Clear => {
                db.clear();
                println!("OK");
            }
            Command::Exists { key } => {
                if db.exists(&key) {
                    println!("true");
                } else {
                    println!("false");
                }
            }
            Command::Count => {
                println!("{}", db.count());
            }
            Command::Help => {
                print_help();
            }
            Command::Unknown(input) => {
                println!("Unrecognized command '{input}'");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Database;
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
}
