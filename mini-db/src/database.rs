use std::collections::HashMap;
use std::io;
use std::path::Path;

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
        // ファイルへ書き込む文字列を作る
        let mut contents = String::new();

        // DBに保存されているデータを1件ずつ取り出す
        for (key, value) in self.list() {
            // key=value\n という形式で文字列へ追加する
            contents.push_str(key);
            contents.push('=');
            contents.push_str(value);
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

        // ファイルの内容を1行ずつ取り出す
        for (line_number, line) in contents.lines().enumerate() {
            // 最初に見つかった「=」を境目として、行番号と合わせてエラーを返す
            let (key, value) = match line.split_once('=') {
                Some(pair) => pair,
                None => {
                    // 「=」がない行は不正な形式として、行番号と合わせてエラーを返す
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("DBファイルの{}行目に「=」がありません", line_number + 1),
                    ));
                }
            };

            // &strをStringに変換してからDBに保存する
            db.set(key.to_string(), value.to_string());
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
