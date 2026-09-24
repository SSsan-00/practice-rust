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

    // keyとvalueの所有権をDBへ私て保存する
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    // keyは読むだけなので借用にする
    // 値がない場合もあるので、Optionを返す
    fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }
}

fn main() -> io::Result<()> {
    // 入力を受け取る
    let mut input = String::new();

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

        // Enterによる改行を取り除いてから判定する
        match input.trim() {
            ".exit" => break,
            // .exit以外の入力をcommandという変数で受け取る
            command => println!("Unrecognized command '{command}'"),
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
}
