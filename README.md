# golden-dawn

Rustを一歩ずつ理解しながら、小さい変更を積み重ねるプロジェクトです。

現在の実装は、`Hello, world!`を1行表示することだけです。
Linuxのクラウド環境でビルド・実行・整形確認済みです。Windowsでは未検証です。

## ビルドと実行

Rustのstableツールチェーン（Cargo・rustfmtを含む）が導入され、`cargo`を実行できる環境が前提です。
導入方法は[公式のRustインストール案内](https://rust-lang.org/tools/install/)を参照してください。

```sh
git clone https://github.com/kwkbhdts/golden-dawn.git
cd golden-dawn
cargo build
cargo run
cargo fmt -- --check
```

プログラムの出力：

```text
Hello, world!
```

## ブランチとタグの方針

- `main`：約束した動作を確認済みの状態。機能が少なくても構いません。
- `dev`：開発途中の状態。小さく作り、ユーザーがコードと動作を確認した後に`main`へ反映します。
- タグ：リリース時点を示します。
