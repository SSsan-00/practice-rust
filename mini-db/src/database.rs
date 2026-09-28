use crate::storage::Storage;
use std::collections::HashMap;
use std::io;

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

/// キーと値をメモリ上に保存するデータベース
pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    /// 空のデータベースを作成
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// キーと値を保存。キーがすでにあれば値を置き換える
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// キーに対応する値を借用して返す。キーがなければ `None` を返す
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    /// キーと値を削除し、削除した値を返す。キーがなければ `None` を返す
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    /// 保存されているキーと値を、順不同のイテレーターとして返す
    ///
    /// 返される文字列スライスは、このデータベースから借用する
    pub fn list(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.data
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Storageを通して現在のデータを保存
    ///
    /// # Errors
    /// Storageでの書き込みに失敗した場合はエラーになる
    pub fn save(&self, storage: &mut dyn Storage) -> io::Result<()> {
        let mut contents = String::from(FORMAT_HEADER);

        for (key, value) in self.list() {
            contents.push_str(&escape_field(key));
            contents.push('\t');
            contents.push_str(&escape_field(value));
            contents.push('\n');
        }
        storage.write(&contents)
    }

    /// Storageを通してデータベースを読み込む
    ///
    /// データがまだない場合は、空のデータベースを作成して保存する
    ///
    /// # Errors
    /// Storageでの読み書きに失敗した場合や、データ形式が不正な場合はエラーになる
    pub fn load(storage: &mut dyn Storage) -> io::Result<Self> {
        let contents = match storage.read() {
            Ok(contents) => contents,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let db = Self::new();
                db.save(storage)?;
                return Ok(db);
            }
            Err(error) => return Err(error),
        };

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

    /// メモリ上のデータをすべて削除。ファイルには保存しない
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 指定したキーが存在するかどうかを返す
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// 保存されているデータの件数を返す
    pub fn count(&self) -> usize {
        self.data.len()
    }
}
