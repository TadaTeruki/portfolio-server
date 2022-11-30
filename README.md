# portfolio-server

ポートフォリオ用のサーバーです。

## 開発資源

コンパイラ: Rust 1.65.0<br>
データベース: [Cloud Firestore](https://firebase.google.com/products/firestore) by Google

## 実行方法

`cargo run` による実行
```
$ PROJECT_ID=(FirebaseのプロジェクトID) \
  GOOGLE_APPLICATION_CREDENTIALS=(FirebaseのCredentialsの入ったjsonファイルへの相対パス) \
  PRIVATE_KEY=(任意の秘密鍵) \
  cargo run
```
