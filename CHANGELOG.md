# CHANGELOG

ALICE-RTOS-BSP の変更履歴。
日付は YYYY-MM-DD。バージョンは [Semantic Versioning](https://semver.org/lang/ja/) 準拠。

## [Unreleased]

(なし)

## [0.1.0] - 2026-06-05

### 追加

#### BSP (6 クレート)
- `alice-bsp-atoms3` — M5Stack AtomS3 (ESP32-S3-PICO-1, 8MB+8MB, LCD/Button/Grove)
- `alice-bsp-stamps3` — M5StampS3 (ESP32-S3-FN8, 21 GPIO 露出)
- `alice-bsp-atom-lite` — M5Atom Lite (ESP32-PICO-D4 / Xtensa LX6, WS2812/Grove)
- `alice-bsp-cores3` — M5Stack CoreS3 (AXP2101 PMIC / BMI270 / タッチLCD / ES8311 codec)
- `alice-bsp-stickc-plus2` — M5StickC Plus2 (ESP32-PICO-V3-02 / MPU6886 / ST7789v2)
- `alice-bsp-cardputer` — M5Stack Cardputer (ESP32-S3 / 56キー / NS4168 / PDM mic)

#### Examples (5 個)
- `blinky` — ALICE-RTOS 起動確認、10Hz + 1Hz の 2タスク RMS
- `ws2812_demo` — WS2812 RGB LED 虹色循環 (esp-hal-smartled2 0.28 + RMT)
- `imu_edge_demo` — MPU6886/BMI270 I2C + ALICE-RTOS `edge` feature
- `synth_demo` — 44.1kHz サイン波生成、ALICE-RTOS `synth` feature
- `motion_demo` — 50kHz 台形プロファイル軌道、ALICE-RTOS `motion` feature

#### 実機動作確認 (M5Stack AtomS3, MAC `98:88:e0:0f:39:c0`)
- blinky: RMS スケジューラ動作 (5sで tick=50/hb=5)
- ws2812_demo: LED 虹色循環、5sで hue=202
- synth_demo: 実サンプル生成レート 43,478Hz / 目標 44,100Hz (誤差 1.4%)
- imu_edge_demo: I2C scan 動作 (AtomS3 ベース機は IMU 非搭載と確認)

### 環境

- **esp-hal**: 1.1.1 (feature `esp32s3` + `unstable`)
- **esp-backtrace**: 0.19 (`panic-handler`, `println`)
- **esp-println**: 0.17 (`log-04`)
- **esp-bootloader-esp-idf**: 0.5 (`esp_app_desc!()` macro)
- **esp-hal-smartled2**: 0.28 (lib名は `esp_hal_smartled`)
- **smart-leds**: 0.4
- **alice-rtos**: 0.1.0 path 依存 (feature `esp32` + 例によって `synth`/`motion`/`edge`)
- **rust-toolchain**: `esp` (espup 1.95.0.0、`espup install --targets esp32,esp32s3`)

### 既知の罠 (CLAUDE.md / memory 参照)

1. App Descriptor 必須 (`esp_bootloader_esp_idf::esp_app_desc!()` を main.rs に)
2. esp-hal 1.x の `delay`/`main` は `unstable` feature 配下
3. esp-hal 0.23 + esp-bootloader-esp-idf 0.5 はシンボル衝突
4. `esp-hal-smartled2` パッケージの lib 名は `esp_hal_smartled` (末尾 "2" なし)
5. M5Stack AtomS3 ベース機は IMU 非搭載 (AtomS3R のみ BMI270 内蔵)
