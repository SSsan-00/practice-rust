use std::collections::HashMap;
use std::io;
use std::path::Path;

const FORMAT_HEADER: &str = "mini-db-v2\n";

// タブ・改行・バックスラッシュを1行に保存できる形へ変換する
fn escape_field(value: &str) -> String {
    let mut escaped = String::new();

    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

// 保存用の表記を元の文字列に戻す
fn unescape_field(value: &str) -> io::Result<String> {
    let mut chars = value.chars();
    let mut unescaped = String::new();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            unescaped.push(ch);
            continue;
        }

        match chars.next() {
            Some('\\') => unescaped.push('\\'),
            Some('n') => unescaped.push('\n'),
            Some('r') => unescaped.push('\r'),
            Some('t') => unescaped.push('\t'),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "DBファイルに不正なエスケープがあります",
                ));
            }
        }
    }

    Ok(unescaped)
}

// DBがデータを所有する
pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    // 空のDBを作成
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // keyとvalueの所有権をDBへ渡して保存する
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    // keyは読むだけなので借用にする
    // 値がない場合もあるので、Optionを返す
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    // 対応する値を削除する
    // 削除する値を返し、存在しなければNoneを返す
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    // DB内のデータを返す
    // 順番の保証はなし
    // Iteratorとして使える値を返す
    // 値(Item)を取り出した時の中身は(&str, &str)
    // '_ self(DB)より長く生き残らない
    pub fn list(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.data
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    // DBの内容をファイルに保存する
    pub fn save(&self, path: &str) -> io::Result<()> {
        // ヘッダーからファイルを作成する
        let mut contents = String::from(FORMAT_HEADER);

        // DBに保存されているデータを1件ずつ取り出す
        for (key, value) in self.list() {
            // key=value\n という形式で文字列へ追加する
            contents.push_str(&escape_field(key));
            contents.push('\t');
            contents.push_str(&escape_field(value));
            contents.push('\n');
        }

        // 指定されたファイルへ文字列を書き込む
        std::fs::write(path, contents)?;

        Ok(())
    }

    // ファイルからDBを読み込む
    pub fn load(path: &str) -> io::Result<Self> {
        // DBファイルがまだ存在しない場合は、空のDBを作成する
        if !Path::new(path).exists() {
            let db = Self::new();
            db.save(path)?;
            return Ok(db);
        }

        // ファイル全体を文字列として埋め込む
        let contents = std::fs::read_to_string(path)?;

        // 空のDBを作る
        let mut db = Self::new();

        // ヘッダーがあればエスケープ文字列対応版として読み込む
        if let Some(contents) = contents.strip_prefix(FORMAT_HEADER) {
            for (line_number, line) in contents.lines().enumerate() {
                // タブを境目にkey, valueを分ける
                let (key, value) = match line.split_once('\t') {
                    Some(pair) => pair,
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("DBファイルの{}行目にタブがありません", line_number + 1),
                        ));
                    }
                };
                // エスケープされた文字列を元に戻してDBに保存する
                db.set(unescape_field(key)?, unescape_field(value)?);
            }
        } else {
            // ヘッダーがなければkey=vlaue形式で読み込む

            for (line_number, line) in contents.lines().enumerate() {
                // タブを境目にkey, valueを分ける
                let (key, value) = match line.split_once('=') {
                    Some(pair) => pair,
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("DBファイルの{}行目に「=」がありません", line_number + 1),
                        ));
                    }
                };
                // エスケープされた文字列を元に戻してDBに保存する
                db.set(key.to_string(), value.to_string());
            }
        }

        // contentsの各行を読み取り、DBへ登録する
        Ok(db)
    }

    // DBに保存されているデータを全て削除する
    pub fn clear(&mut self) {
        self.data.clear();
    }

    // 指定したキーがDBに存在するか確認する
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    // 保存されているデータ件数を返す
    pub fn count(&self) -> usize {
        self.data.len()
    }
}
