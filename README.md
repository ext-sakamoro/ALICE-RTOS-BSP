# ALICE-RTOS-BSP

ALICE-RTOS 用ボード支援パッケージ群 (Board Support Package)。
各 M5Stack / ESP32 / Cortex-M / RISC-V ボードと ALICE-RTOS を結ぶ薄いグルー層。

## 対応ボード

| ボード | クレート | チップ | 状態 |
|--------|---------|--------|------|
| **M5Stack AtomS3** | `alice-bsp-atoms3` | ESP32-S3-PICO-1 | 🟢 動作確認中 |
| M5Stack CoreS3 | `alice-bsp-cores3` | ESP32-S3 | ⚪ 予定 |
| M5StampS3      | `alice-bsp-stamps3` | ESP32-S3 | ⚪ 予定 |
| M5Atom Lite    | `alice-bsp-atom-lite` | ESP32 | ⚪ 予定 |

## 設計方針

- **ALICE-RTOS 本体に依存を持ち込まない** — `esp-hal` / `m5stack` 等は全部こちら側
- **ピン定数とクロックのみ提供** — ペリフェラル抽象は esp-hal を直接使う
- **examples/ で動く完全デモ** — 「とりあえず flash して動く」が必ず手元にある

## 構成

```
ALICE-RTOS-BSP/
├── Cargo.toml                # workspace
├── rust-toolchain.toml       # channel = "esp" (espup 必須)
├── .cargo/config.toml        # target = xtensa-esp32s3-none-elf
├── crates/
│   └── alice-bsp-atoms3/
└── examples/
    └── blinky/               # 2タスク (10Hz + 1Hz) RMS デモ
```

## ビルド & フラッシュ (AtomS3)

```bash
# 初回のみ: ESP32-S3 用 Rust toolchain (espup)
cargo install espup espflash
espup install --targets esp32s3
. $HOME/export-esp.sh

# 例の書き込み
cd examples/blinky
cargo run --release
```

`espflash` が `/dev/cu.usbmodem*` を自動検出して flash + serial monitor を開始する。

## 期待される出力

```
============================================
 ALICE-RTOS-BSP / AtomS3 (ESP32-S3-PICO-1)
 Math-First RTOS booting...
============================================
Tasks registered. Entering scheduler loop @ 1ms tick.
[t=   5s] ticks=50 hb=5
[t=  10s] ticks=100 hb=10
...
```

10Hz タスクが 5秒で 50回、1Hz タスクが 5秒で 5回スケジュール実行されれば RMS が機能している。

## ライセンス

MIT OR Apache-2.0
