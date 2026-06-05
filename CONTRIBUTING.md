# CONTRIBUTING

ALICE-RTOS-BSP への貢献ガイド。

## 開発環境

```bash
# 初回のみ
cargo install espup espflash
espup install --targets esp32,esp32s3   # ESP32-S3 (AtomS3 等) + ESP32 LX6 (Atom Lite)
. ~/export-esp.sh

# 親プロジェクト ALICE-RTOS を path 依存で参照 (../ALICE-RTOS)
git clone https://github.com/ext-sakamoro/ALICE-RTOS ../ALICE-RTOS
```

## ビルド・テスト

```bash
cargo build --release                # 全パッケージ
cargo build --release -p ws2812-demo # 例単体
```

## 実機 flash

```bash
cd examples/ws2812_demo
cargo run --release   # espflash flash --monitor を自動実行
```

## BSP 追加ルール

新ボードの BSP を追加する場合:

1. `crates/alice-bsp-<board>/` を作成
2. `Cargo.toml` の `description` は「Board Support Package for <board> (chip) with ALICE-RTOS」
3. `src/lib.rs` は `#![no_std]` 必須、`alice-rtos` / `esp-hal` 依存禁止 (BSP は const のみ)
4. ピン定数は `pub mod pinout` 配下に整理
5. `CPU_HZ` 定数必須
6. I2C デバイスは `<DEVICE>_I2C_ADDR` 定数で公開
7. `Cargo.toml` workspace members に追加
8. README.md / CHANGELOG.md に行追加

## Example 追加ルール

1. `examples/<name>/` を作成
2. `alice-rtos` のみ feature を追加するときは `{ workspace = true, features = ["xxx"] }`
3. `esp_bootloader_esp_idf::esp_app_desc!();` を main.rs 先頭に書く (必須、忘れると bootloop)
4. esp-hal は `features = ["esp32s3", "unstable"]` (workspace 既定で OK)
5. 実機確認したらこのファイルと CHANGELOG に実機情報を記載

## コミット

- 作成者: `Moroya Sakamoto <sakamoro@alicelaw.net>`
- メッセージ: 日本語、短く淡々と。署名・Co-Authored-By 等は禁止
- `CLAUDE.md` / `.claude/` はリモートに push しない (`.gitignore` 済)

## License

MIT OR Apache-2.0
