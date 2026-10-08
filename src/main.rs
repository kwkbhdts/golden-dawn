use std::path::PathBuf;

/// このプログラムの実行ファイルのパスを返す。
/// OSの呼び出しやファイルシステム操作などで取得に失敗すると、エラーを返す。
fn executable_path() -> std::io::Result<PathBuf> {
    std::env::current_exe()
}

/// 現在のローカル時刻の日付をYYYY-MM-DDの文字列で返す。
fn current_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// パスと日付を表示し、成功時はOk(())を返す。パス取得失敗時は?でエラーを返す。
fn main() -> std::io::Result<()> {
    let path = executable_path()?;
    let date = current_date();
    println!("{}", path.display());
    println!("{date}");
    Ok(())
}
