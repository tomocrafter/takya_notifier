takya_notifier
===

steamrmt.com をスクレイピングし、fcmを通してスマホや、webpushでPCに通知を送ることができるツールです。

## Build
二通りのビルド方法を用意しています。実行時のエラーを収集、解析または通知するために[Sentry](https://sentry.io/)がデフォルトで入っています。

そのため、Sentryを利用する場合は

`cargo build --release`

利用しない場合は

`cargo build --release --no-default-features`

でビルドができます。

エラーが出る場合:
* libmysqlclient エラー: MySQLクライアントが必要です。mysql-develやmysql-libsをインストールしてください。

## Usage
環境変数を設定する必要があります。詳しくは `.env.example` をコピーして内容を変更してください。

なお、Sentryを利用しない場合は `SENTRY_DSN` は必要ありません。コメントアウトしてください。

使い方はビルドされた `target/release/takya_notifier` を実行するだけです。
