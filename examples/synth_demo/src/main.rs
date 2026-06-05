#![no_std]
#![no_main]

//! Synth demo (ALICE-Synth 簡易模倣)
//!
//! 外付け I2S DAC (例: PCM5102A on Grove I2S DAC, ES8311 拡張ボード等) を AtomS3 の
//! Grove ポート (G1=GPIO2, G2=GPIO1) 経由で接続する想定。
//!
//! - 44.1kHz サンプリング、モノラル 16-bit
//! - 480Hz サイン波を ring buffer に書き込み、I2S が DMA 経由で吐き出す
//! - ALICE-RTOS は 22.7µs (44.1kHz) 周期のレンダリングタスクをスケジュール
//!
//! 本例は **ビルド検証のみ**。実機テストは外付け DAC 必須。
//! AtomS3 の GPIO 配置上、Grove ポートの 2ピンしかないため I2S 全 3 信号 (BCK/WS/DIN)
//! には足りない。CoreS3 で完全動作を狙う場合に StampS3 へ移植推奨。

use alice_bsp_atoms3::CPU_HZ;
use alice_rtos::{Kernel, TaskPriority};
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_app_desc!();

const SAMPLE_RATE_HZ: u32 = 44_100;
const TONE_HZ:        u32 = 480;
/// 22.7µs per sample (44100Hz) — ALICE-Synth 既定
const SAMPLE_PERIOD_US: u32 = 1_000_000 / SAMPLE_RATE_HZ;

static PHASE_Q15: AtomicU32 = AtomicU32::new(0);
static LAST_SAMPLE: AtomicI32 = AtomicI32::new(0);

/// 22.7µs 周期で 1 サンプル生成 (本来は I2S DMA 半分割込で呼ばれる)
fn task_render_sample(_scratch: &mut [u8]) {
    // 位相増分: TONE_HZ * 65536 / SAMPLE_RATE_HZ
    let phase_inc = (TONE_HZ << 16) / SAMPLE_RATE_HZ;
    let phase = PHASE_Q15.fetch_add(phase_inc, Ordering::Relaxed);
    // 線形近似で正弦値 (簡略 — 本物の ALICE-Synth は FMA wave table)
    let p = (phase >> 8) as i32; // 0..255
    let s = if p < 128 {
        p * 256 - 128 * 128
    } else {
        (256 - p) * 256 - 128 * 128
    };
    LAST_SAMPLE.store(s, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let _p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS Synth demo @ AtomS3 (no DAC attached) ===");
    println!(" SAMPLE_RATE={}Hz, TONE={}Hz, period={}us",
             SAMPLE_RATE_HZ, TONE_HZ, SAMPLE_PERIOD_US);

    let mut kernel = Kernel::new(CPU_HZ);
    kernel
        .add_task(b"render", task_render_sample, TaskPriority::CRITICAL,
                  SAMPLE_PERIOD_US, 5)
        .expect("render task");

    println!("44.1kHz render task scheduled — attach I2S DAC to Grove for real output");

    let mut last_log_us: u64 = 0;
    loop {
        // 本来 1µs tick が欲しいが espflash monitor の出力負荷で破綻するので 10µs に間引く
        kernel.tick(10);
        delay.delay_micros(10);
        let now_us = kernel.total_ticks * 10;
        if now_us - last_log_us >= 1_000_000 {
            last_log_us = now_us;
            println!("[t={}s] last_sample={} phase={}",
                     now_us / 1_000_000,
                     LAST_SAMPLE.load(Ordering::Relaxed),
                     PHASE_Q15.load(Ordering::Relaxed));
        }
    }
}
