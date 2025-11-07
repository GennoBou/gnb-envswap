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

2.  **PowerShellで `envswap` を実行:**

    Scoopでインストールした場合、便利な `envswap` 関数がPowerShellプロファイルに自動で追加されます。ターミナルで `envswap` を実行してください。

    ```powershell
    envswap
    ```

    TUIが起動したら、矢印キーで移動し、`Enter`キーで選択します。変数と値を選ぶと、現在のPowerShellセッションに環境変数が適用されます。

    **仕組み（と手動設定の方法）:**

    `envswap` 関数は、内部で `gnb-envswap | Invoke-Expression` を実行するラッパーです。`gnb-envswap` コマンドは変数を設定するためのPowerShellコマンドを生成し、それを `Invoke-Expression` が実行することで設定が適用されます。

    Scoopを使わずにインストールした場合は、以下のコマンドで直接利用できます。

    ```powershell
    gnb-envswap | Invoke-Expression
    ```

    また、利便性のために `envswap` 関数を自分でPowerShellプロファイル（`$PROFILE`）に登録することも可能です。

### `show` サブコマンド

`show` サブコマンドは、設定ファイルに定義された環境変数の現在の状態を確認するために使用します。

```powershell
# 現在の状態を表示（値はマスクされます）
gnb-envswap show
```

出力例:
```
API_KEY: 開発サーバー 🚀
DB_HOST: 未設定
SECRET_TOKEN: 設定外の値
```

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルをご覧ください。
