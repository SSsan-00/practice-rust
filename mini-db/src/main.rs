use std::io::{self, Write};

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
