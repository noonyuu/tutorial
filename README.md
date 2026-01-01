# Rust

## 導入
brew install rustup-init

## 環境

### プロジェクト作成
cargo new tutorial

### 実行
cargo run

### main以外を実行
Cargo.tomlに以下を追加
```
[[bin]]
name = "ファイル名"
path = "Cargo.tomlから見た相対パス"

cargo run --bin ファイル名

## 変数と可変性
