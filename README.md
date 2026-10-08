# golden-dawn

Rustを一歩ずつ理解しながら、小さい変更を積み重ねるプロジェクトです。

現在の実装は、起動された実行ファイル自身のパスを取得して1行表示することです。
Linuxのクラウド環境でビルド・実行・整形確認済みです。
Windows/LinuxのCIを用意しています。Windows実機では未確認です。

## ビルドと実行

Rustのstableツールチェーン（Cargo・rustfmtを含む）が導入され、`cargo`を実行できる環境が前提です。
導入方法は[公式のRustインストール案内](https://rust-lang.org/tools/install/)を参照してください。

```sh
git clone https://github.com/kwkbhdts/golden-dawn.git
cd golden-dawn
git switch dev
cargo build
cargo run
cargo fmt -- --check
```

プログラムの出力例（Linux。配置場所によって変わります）：

```text
/path/to/golden-dawn/target/debug/golden-dawn
```

`executable_path()`は`std::env::current_exe()`で取得したパスを`PathBuf`で返し、取得失敗を`Result`でmainへ伝えます。
作業ディレクトリ（cwd）や起動引数`argv[0]`の文字列をそのまま返す関数ではありません。
パスの表現やシンボリックリンク経由で起動した場合の返値はOSによって異なります。[公式APIの説明](https://doc.rust-lang.org/std/env/fn.current_exe.html)

## Windows/Linuxのビルド

対象はx86_64です。今回の設定とCIは`dev`で開発しています。

| OS | ターゲット | ビルド時と実行時の依存 |
| --- | --- | --- |
| Windows | `x86_64-pc-windows-msvc` | ビルドにはMSVC Build ToolsとWindows SDKが必要です。`.cargo/config.toml`の`crt-static`でCRTを静的リンクし、exe実行時のVC++再頒布ランタイムDLLへの依存を減らします。WindowsのシステムDLLは必要です。 |
| Linux | `x86_64-unknown-linux-gnu` | 標準のGNUターゲットを使い、追加のクロスツールチェーンを避けます。glibcへの動的依存が残ります。 |

Windows GNUはMSVCの代わりにMinGW/GCCを使う候補ですが、今回は標準runnerに用意されたMSVCを利用し、配布exeのCRTを静的にする構成を選びました。
MSVC静的CRTはビルド時のVisual C++ツールを不要にする設定ではありません。
根拠：[RustのCRTリンク仕様](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes)、[Windows GNUターゲット](https://doc.rust-lang.org/rustc/platform-support/windows-gnu.html)、[Microsoftの静的CRT説明](https://learn.microsoft.com/en-us/cpp/build/reference/md-mt-ld-use-run-time-library?view=msvc-170)。

ターゲットを導入したうえで、使用するOSのコマンドを実行します。

```sh
# Linux
rustup target add x86_64-unknown-linux-gnu
cargo build --locked --release --target x86_64-unknown-linux-gnu
./target/x86_64-unknown-linux-gnu/release/golden-dawn
```

```powershell
# Windows
rustup target add x86_64-pc-windows-msvc
cargo build --locked --release --target x86_64-pc-windows-msvc
.\target\x86_64-pc-windows-msvc\release\golden-dawn.exe
```

## CIと確認範囲

[Buildワークフロー](https://github.com/kwkbhdts/golden-dawn/actions/workflows/build.yml)は、`main`/`dev`へのpushと、それらに向けたPRで実行します。
公開リポジトリの標準GitHub-hosted runner（Ubuntu 24.04・Windows Server 2022）を使い、Rust 1.99.0で整形確認、`--locked`のreleaseビルドを行い、表示されたパスが生成した実行ファイルを指すことを確認します。
Windowsは`dumpbin`で直接インポートするDLLを調べ、CRTランタイムDLLがあれば失敗させます。Linuxは動的ライブラリと要求するglibc版をログに表示します。
各OSの実行ファイルを3日間のCI成果物として保存します。Linuxの成果物は展開後に実行権限の付与が必要になる場合があります。

Windows runnerにはVC++ランタイムがあるため、実行成功だけでは再頒布ランタイム不要の証明になりません。DLL検査は現在の標準ライブラリだけのプログラムの直接インポート確認であり、Windows実機やランタイム未導入PCでの実行は未確認です。
Windows 10以降がRustターゲットの対象ですが、CIで確認するOSはWindows Server 2022のみです。
LinuxはUbuntu 24.04のCIとクラウド環境での確認に限ります。生成したバイナリにはビルド環境由来のglibc要件があるため、古いglibc環境やAlpineなどのmusl環境での互換性は保証しません。
他のCPUアーキテクチャも未確認です。[Rustの対応プラットフォーム](https://doc.rust-lang.org/rustc/platform-support.html)

## ブランチとタグの方針

- `main`：約束した動作を確認済みの状態。機能が少なくても構いません。
- `dev`：開発途中の状態。小さく作り、ユーザーがコードと動作を確認した後に`main`へ反映します。
- タグ：リリース時点を示します。
