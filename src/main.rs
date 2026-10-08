use std::path::PathBuf;

/// このプログラムの実行ファイルのパスを返す。
/// OSの呼び出しやファイルシステム操作などで取得に失敗すると、エラーを返す。
fn executable_path() -> std::io::Result<PathBuf> {
    std::env::current_exe()
}

/// パスを表示し、成功時はOk(())を返す。取得失敗時は?でエラーを返す。
fn main() -> std::io::Result<()> {
    let path = executable_path()?;
    println!("{}", path.display());
    Ok(())
}
