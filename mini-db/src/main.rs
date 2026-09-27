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

/// 使用可能なコマンドと用途を表示
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

    // DBファイルからデータを読み込む
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
mod tests;
