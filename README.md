# ALICE-RTOS-BSP

ALICE-RTOS 用ボード支援パッケージ群 (Board Support Package)。
各 M5Stack / ESP32 / Cortex-M / RISC-V ボードと ALICE-RTOS を結ぶ薄いグルー層。

## 対応ボード (BSP)

| ボード | クレート | チップ | 実機検証 |
|--------|---------|--------|---------|
| **M5Stack AtomS3** | `alice-bsp-atoms3` | ESP32-S3-PICO-1 | 🟢 動作確認済 (MAC `98:88:e0:0f:39:c0`) |
| M5StampS3 | `alice-bsp-stamps3` | ESP32-S3-FN8 | ⚪ ピン定数のみ (ビルドOK) |
| M5Atom Lite | `alice-bsp-atom-lite` | ESP32-PICO-D4 (Xtensa LX6) | ⚪ ピン定数のみ |

## サンプル例 (examples/)

| 例 | 内容 | 検証状況 |
|----|------|---------|
| **blinky** | ALICE-RTOS 起動確認、10Hz+1Hz の 2タスク RMS | 🟢 実機 |
| **ws2812_demo** | WS2812 RGB LED 虹色循環 (20Hz hue + 33Hz render) | 🟢 実機 |
| **imu_edge_demo** | MPU6886/BMI270 + 線形傾き分類器 (100Hz) | ⚪ I2C scan 確認、IMU は AtomS3R or 外付け要 |
| **synth_demo** | I2S DAC 外付け前提 ALICE-Synth 44.1kHz サイン波 | ⚪ ビルドのみ |
| **motion_demo** | GPIO ステッパ 10kHz 台形プロファイル軌道 | ⚪ ビルドのみ |

## 設計方針

- **ALICE-RTOS 本体に依存を持ち込まない** — `esp-hal` / `smart-leds` 等は example 側で直接
- **BSP は const + 軽量ヘルパのみ** — ピン定数 / CPU クロック / I2C アドレス
- **examples/ で動く完全デモ** — 「とりあえず flash して動く」を全例で担保

## 構成

```
ALICE-RTOS-BSP/
├── Cargo.toml                # workspace
├── rust-toolchain.toml       # channel = "esp" (espup 必須)
├── .cargo/config.toml        # target = xtensa-esp32s3-none-elf
├── crates/
│   ├── alice-bsp-atoms3/     # ESP32-S3-PICO-1
│   ├── alice-bsp-stamps3/    # ESP32-S3-FN8
│   └── alice-bsp-atom-lite/  # ESP32-PICO-D4 (LX6)
└── examples/
    ├── blinky/
    ├── ws2812_demo/
    ├── imu_edge_demo/
    ├── synth_demo/
    └── motion_demo/
```

## ビルド & フラッシュ (AtomS3)

```bash
# 初回のみ: ESP32-S3 用 Rust toolchain (espup)
cargo install espup espflash
espup install --targets esp32,esp32s3   # atom-lite 用に esp32 も
. $HOME/export-esp.sh

# blinky の例
cd examples/blinky
cargo run --release

# WS2812 LED の例
cd ../ws2812_demo
cargo run --release
```

`espflash` が `/dev/cu.usbmodem*` を自動検出して flash + serial monitor を開始する。

## 期待される出力例

### blinky (RMS スケジューラ動作確認)
```
ALICE-RTOS-BSP / AtomS3 (ESP32-S3-PICO-1)
Tasks registered. Entering scheduler loop @ 1ms tick.
[t=   5s] ticks=51 hb=5
[t=  10s] ticks=101 hb=10
```

### ws2812_demo
```
=== ALICE-RTOS WS2812 demo @ AtomS3 ===
RGB hue loop running, 20Hz hue task + 33Hz render
[t=5s] hue=202
```
(LED が赤 → 緑 → 青 → 紫 → 赤 の虹色を循環)

## バイナリサイズ

| 例 | size | flash 占有 |
|----|------|-----------|
| blinky | 87,552 B | 1.05% / 8MB |
| ws2812_demo | 91,920 B | 1.10% |
| imu_edge_demo | 99,472 B | 1.20% |

ALICE-RTOS 自体は < 2KB なので、残りは esp-hal HAL + esp-println。

## 依存バージョン整合性 (重要)

esp-hal 系は版がズレるとシンボル衝突する。以下で動作確認済み:

| crate | version | feature |
|-------|---------|---------|
| `esp-hal` | 1.1.1 | `esp32s3`, **`unstable`** |
| `esp-backtrace` | 0.19 | `esp32s3`, `panic-handler`, `println` |
| `esp-println` | 0.17 | `esp32s3`, `log-04` |
| `esp-bootloader-esp-idf` | 0.5 | `esp32s3` |
| `esp-hal-smartled2` | 0.28 | (lib名: `esp_hal_smartled`、"2"なし) |
| `smart-leds` | 0.4 | — |

## 既知の罠

1. **App Descriptor 必須**: `espflash flash` は `esp_bootloader_esp_idf::esp_app_desc!()` をバイナリに埋め込み必須。`--ignore-app-descriptor` 回避すると bootloop
2. **esp-hal `unstable` feature**: 1.x の `delay` / `main` 等は unstable 配下
3. **AtomS3 ベース機は IMU 非搭載**: AtomS3R のみ BMI270 内蔵 (I2C: SDA=GPIO38, SCL=GPIO39, addr=0x69)
4. **esp-hal-smartled2 のlib名**: パッケージ名 `esp-hal-smartled2` だが lib名は `esp_hal_smartled` (末尾 "2" なし)

## ライセンス

MIT OR Apache-2.0
