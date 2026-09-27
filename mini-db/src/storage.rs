use std::io;
use std::path::PathBuf;

/// 文字列データを読み書きする保存先の共通ルール
pub trait Storage {
    /// 保存先から内容を読み込む
    fn read(&self) -> io::Result<String>;

    /// 保存先へ内容を書き込む
    fn write(&mut self, contents: &str) -> io::Result<()>;
}

/// ファイルを保存先として使う
pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    /// 指定したパスを使うファイル保存先を作る
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Storage for FileStorage {
    fn read(&self) -> io::Result<String> {
        std::fs::read_to_string(&self.path)
    }

    fn write(&mut self, contents: &str) -> io::Result<()> {
        std::fs::write(&self.path, contents)
    }
}
