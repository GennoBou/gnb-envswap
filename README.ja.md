# gnb-envswap

[English](./README.md) | 日本語

`gnb-envswap`は、PowerShellセッションで利用する環境変数を、テキストベースのUI（TUI）を通じて素早く・安全に切り替えるためのコマンドラインツールです。APIキーやデータベース接続先など、環境に依存する設定を頻繁に切り替える開発者のワークフローを効率化するために作られました。

## 特徴

-   **インタラクティブなTUI:** 環境変数とその値を簡単に選択できる、ユーザーフレンドリーなインターフェース。
-   **TOMLによる設定:** `.env.swap.toml` ファイルに、環境変数のセットをシンプルに定義できます。
-   **Rust製で高速:** Rustでビルドされた、単一で軽量な実行ファイルです。
-   **国際化対応:** UIメッセージは英語と日本語に対応しています（OSのロケールから自動判別）。

## インストール

Windowsへのインストールは、[Scoop](https://scoop.sh/) を利用する方法を推奨しています。

```powershell
# 開発者のBucketを追加します（実際のURLに置き換えてください）
scoop bucket add gennobou https://github.com/gennobou/scoop-bucket
# アプリをインストールします
scoop install gennobou/gnb-envswap
```

## 使い方

1.  **設定ファイルの作成:**

    プロジェクトのルートディレクトリ、またはホームディレクトリ（`~`）に `.env.swap.toml` という名前のファイルを作成します。

    ```toml
    [API_KEY]
    [[API_KEY.values]]
    label = "開発サーバー 🚀"
    value = "dev_api_key_xxxxxxxxx"

    [[API_KEY.values]]
    label = "本番サーバー"
    value = "prod_api_key_yyyyyyyy"

    [DB_HOST]
    [[DB_HOST.values]]
    label = "ローカルデータベース"
    value = "localhost"
    ```

2.  **PowerShellで実行:**

    PowerShellターミナルで `gnb-envswap` を実行すると、環境変数を選択するためのTUIが表示されます。

    ```powershell
    gnb-envswap
    ```

3.  **選択と適用:**

    矢印キーで移動し、`Enter`キーで選択します。変数と値を選ぶと、ツールはPowerShellコマンドを標準出力します。これを現在のセッションに適用するには、`Invoke-Expression` を使います。

    ```powershell
    gnb-envswap | Invoke-Expression
    ```

    毎回入力するのが面倒な場合は、PowerShellの関数として登録しておくと便利です。

    ```powershell
    function envswap {
        gnb-envswap | Invoke-Expression
    }
    ```

    これで、`envswap` を実行するだけでよくなります。

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルをご覧ください。
