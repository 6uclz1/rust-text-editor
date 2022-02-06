use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

/// CTRL + `AnyKey` のバイトコードを返却する
/// # Arguments
///
/// * `c` - 文字 `[a-z,A-Z,0-9]`
///
fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

/// panic! で強制的にプログラムを終了させる
/// # Arguments
///
/// * `e` - エラー
///
fn die(e: std::io::Error) {
    panic!("{}", e);
}

/// メイン関数
fn main() {
    // キーボードから入力された文字列(使用してない)
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {

        match b {
            // バイトコードだった場合
            Ok(b) => {
                // 文字
                let c = b as char;

                // 制御文字の場合
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    // 制御文字ではない([a-z,A-Z,0-9]など)の場合
                    println!("{:?} ({})\r", b, c);
                }
                // ctrl + 'q' を受け取ったら終了する
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            // エラーだった場合
            Err(err) => die(err),
        }
    }
}
