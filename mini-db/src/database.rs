use std::collections::HashMap;
use std::io;
use std::path::Path;

const FORMAT_HEADER: &str = "mini-db-v2\n";

// タブ・復帰文字・改行・バックスラッシュを1行に保存できる形へ変換する
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

/// キーと値をメモリ上に保存するデータベースです。
pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    /// 空のデータベースを作成します。
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// キーと値を保存します。すでにキーがある場合は、値を置き換えます。
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// キーに対応する値を借用して返します。キーが見つからない場合は `None` を返します。
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    /// キーと値を削除し、削除した値を返します。キーが見つからない場合は `None` を返します。
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    /// 保存されているキーと値を、順不同のイテレーターとして返します。
    ///
    /// 返される文字列スライスは、このデータベースから借用されています。
    pub fn list(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.data
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// 現在のデータを指定したファイルへ保存します。
    ///
    /// # Errors
    /// ファイルへ書き込めない場合はエラーを返します。
    pub fn save(&self, path: &str) -> io::Result<()> {
        // ヘッダーを保存内容の先頭に設定する
        let mut contents = String::from(FORMAT_HEADER);

        // DBに保存されているデータを1件ずつ取り出す
        for (key, value) in self.list() {
            contents.push_str(&escape_field(key));
            contents.push('\t');
            contents.push_str(&escape_field(value));
            contents.push('\n');
        }

        // 指定されたファイルへ文字列を書き込む
        std::fs::write(path, contents)?;

        Ok(())
    }

    /// 指定したファイルからデータベースを読み込みます。
    ///
    /// ファイルが存在しない場合は、空のデータベースを作成して保存します。
    ///
    /// # Errors
    /// ファイルを読み書きできない場合や、ファイル形式が不正な場合はエラーを返します。
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
                            format!("DBファイルの{}行目にタブがありません", line_number + 2),
                        ));
                    }
                };
                // エスケープされた文字列を元に戻してDBに保存する
                db.set(unescape_field(key)?, unescape_field(value)?);
            }
        } else {
            // ヘッダーがなければkey=value形式で読み込む

            for (line_number, line) in contents.lines().enumerate() {
                // 「=」を境目にkey, valueを分ける
                let (key, value) = match line.split_once('=') {
                    Some(pair) => pair,
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("DBファイルの{}行目に「=」がありません", line_number + 1),
                        ));
                    }
                };
                db.set(key.to_string(), value.to_string());
            }
        }

        // 読み込みが完了したDBを返す
        Ok(db)
    }

    /// メモリ上のデータをすべて削除します。ファイルへの保存は行いません。
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 指定したキーが存在するかどうかを返します。
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// 保存されているデータの件数を返します。
    pub fn count(&self) -> usize {
        self.data.len()
    }
}
