`gnb-envswap` は、PowerShellセッションで環境変数を素早く・安全に切り替えるための、TUI (Text-based UI) を備えたコマンドラインツールです。異なるAPIキーやデータベース接続先などを頻繁に切り替えて開発・運用するエンジニアのワークフローを効率化します。

## 主な機能

-   **インタラクティブなTUI:** 矢印キーと文字入力で直感的に環境変数と値を選択できます。
-   **リアルタイム検索:** 文字を入力するだけで、インクリメンタルに項目を検索・絞り込みできます。
-   **TOMLによる設定:** `.env.swap.toml` ファイルで設定をシンプルに定義できます。
-   **スマートな設定マージ:** ワークディレクトリとホームディレクトリの設定を自動的に統合し、`<Work>` と `<Home>` の色付きプレフィックスで出所を明確に区別します。
-   **Rust製の高速動作:** Rust 2024 Editionで構築された、単一で軽量・高速な実行ファイルです。
-   **i18n対応:** UIメッセージは英語と日本語をサポートしています（OSのロケールから自動判定）。

## インストール

Windowsでの推奨インストール方法は [Scoop](https://scoop.sh/) を使用することです。

```powershell
# 開発者のバケットを追加 (実際のバケットURLに置き換えてください)
scoop bucket add gennobou https://github.com/gennobou/scoop-bucket
# アプリのインストール
scoop install gennobou/gnb-envswap
```

## 使い方

1.  **設定ファイルの作成:**
    `.env.swap.toml` を作成します。詳細な設定方法や、カレントディレクトリとホームディレクトリのマージ仕様については、[設定ファイルリファレンス](docs/configuration.md) を参照してください。

2.  **PowerShellで `envswap` を実行:**

    Scoopでインストールした場合、便利な `envswap` 関数がPowerShellプロファイルに自動的に追加されます。単に `envswap` と実行するだけです。

    ```powershell
    envswap
    ```

    TUIが開きます。
    *   **文字入力** でリストを検索・フィルタリングします。
    *   **↑ / ↓ キー** で項目を移動します（リストはループします）。
    *   **Enter キー** で決定します。
    *   **Esc キー** で戻る、または終了します。

    **動作原理 (と手動セットアップ):**

    `envswap` 関数は、単に `gnb-envswap | Invoke-Expression` を実行するラッパーです。`gnb-envswap` コマンド自体は変数を設定するためのPowerShellコマンドを出力し、それを `Invoke-Expression` が適用します。

    Scoopを使わずにインストールした場合は、フルコマンドを実行して使用できます:

    ```powershell
    gnb-envswap | Invoke-Expression
    ```

    または、ご自身のPowerShellプロファイル (`$PROFILE`) に手動で `envswap` 関数を追加してください。

### `show` サブコマンド

`show` サブコマンドは、設定ファイルに定義されている環境変数の現在の状態を確認するために使用します。誤って `Invoke-Expression` にパイプされるのを防ぐため、出力はすべて標準エラー出力 (`stderr`) に送られます。

```powershell
# 現在の状態を表示 (値はマスクされます)
gnb-envswap show
```

出力例:
```text
API_KEY: 開発環境 (Dev) 🚀
DB_HOST: 未設定
SECRET_TOKEN: 設定外の値
```

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルをご覧ください。
