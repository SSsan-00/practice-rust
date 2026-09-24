use std::collections::HashMap;
use std::io::{self, Write};

// DBがデータを所有する
struct Database {
    data: HashMap<String, String>,
}

impl Database {
    // 空のDBを作成
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // keyとvalueの所有権をDBへ渡して保存する
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    // keyは読むだけなので借用にする
    // 値がない場合もあるので、Optionを返す
    fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    // 対応する値を削除する
    // 削除する値を返し、存在しなければNoneを返す
    fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    // DB内のデータを返す
    // 順番の保証はなし
    // Iteratorとして使える値を返す
    // 値(Item)を取り出した時の中身は(&str, &str)
    // '_ self(DB)より長く生き残らない
    fn list(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.data
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

enum Command {
    Exit,
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    List,
    Unknown(String),
}

fn parse_command(input: &str) -> Command {
    let words: Vec<&str> = input.split_whitespace().collect();

    // words.as_slice(): &[&str]
    match words.as_slice() {
        [".exit"] => Command::Exit,
        ["set", key, value] => Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        },
        ["get", key] => Command::Get {
            key: key.to_string(),
        },
        ["remove", key] => Command::Remove {
            key: key.to_string(),
        },
        ["list"] => Command::List,
        _ => Command::Unknown(input.to_string()),
    }
}

fn main() -> io::Result<()> {
    // 入力を受け取る
    let mut input = String::new();

    // DBを作成する
    let mut db = Database::new();

    loop {
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
            Command::Exit => break,
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
}
