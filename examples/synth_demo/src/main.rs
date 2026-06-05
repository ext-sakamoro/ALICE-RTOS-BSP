#![no_std]
#![no_main]

//! Synth demo — ALICE-RTOS の `synth` feature 正規版
//!
//! `alice_rtos::synth_tasks` モジュールが提供する 44.1kHz 用 task テンプレート
//! `synth_task_default` を使い、CRITICAL 優先度 + 23µs 周期 + 8µs WCET で
//! サンプル生成タスクをスケジュール。
//!
//! 外付け I2S DAC (PCM5102A on Grove I2S DAC、ES8311 拡張ボード等) を AtomS3 の
//! Grove ポートに接続すれば実際に音声出力される。本デモは DAC 非接続前提で
//! サンプル値をシリアル監視で確認するモード。

use alice_bsp_atoms3::CPU_HZ;
use alice_rtos::{synth_tasks, Kernel};
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_app_desc!();

const SAMPLE_RATE_HZ: u32 = 44_100;
const TONE_HZ:        u32 = 480;

static PHASE_Q16:    AtomicU32 = AtomicU32::new(0);
static LAST_SAMPLE:  AtomicI32 = AtomicI32::new(0);
static SAMPLE_COUNT: AtomicU32 = AtomicU32::new(0);

/// 23µs (44.1kHz) 周期で 1サンプル生成
/// ALICE-Synth 風: 線形近似三角波 (実機 ALICE-Synth では FMA wave table)
fn task_render_sample(_scratch: &mut [u8]) {
    let phase_inc = (TONE_HZ << 16) / SAMPLE_RATE_HZ;
    let phase = PHASE_Q16.fetch_add(phase_inc, Ordering::Relaxed);
    let p = (phase >> 8) as i32 & 0xFF; // 0..255
    let s = if p < 128 {
        p * 256 - 128 * 128
    } else {
        (256 - p) * 256 - 128 * 128
    };
    LAST_SAMPLE.store(s, Ordering::Relaxed);
    SAMPLE_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let _p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS Synth demo @ AtomS3 (ALICE-RTOS synth feature) ===");
    println!(
        " SYNTH_PERIOD_US={} ({}Hz), SYNTH_WCET_US={}, PRIORITY=CRITICAL",
        synth_tasks::SYNTH_PERIOD_US,
        1_000_000 / synth_tasks::SYNTH_PERIOD_US,
        synth_tasks::SYNTH_WCET_US,
    );
    println!(
        " 4-voice capacity check: max_voices = {}",
        synth_tasks::max_voices(synth_tasks::SYNTH_PERIOD_US, synth_tasks::SYNTH_WCET_US / 4),
    );

    let mut kernel = Kernel::new(CPU_HZ);
    let synth_task = synth_tasks::synth_task_default(task_render_sample);
    kernel
        .scheduler
        .register(synth_task)
        .expect("synth task register");

    println!("Synth task registered. Tone = {}Hz triangle wave", TONE_HZ);
    println!("(Attach PCM5102A I2S DAC to Grove for audio output)");

    let mut last_log_us: u64 = 0;
    loop {
        kernel.tick(10);
        delay.delay_micros(10);
        let now_us = kernel.total_ticks * 10;
        if now_us - last_log_us >= 1_000_000 {
            last_log_us = now_us;
            let actual_rate = SAMPLE_COUNT.swap(0, Ordering::Relaxed);
            println!(
                "[t={}s] last_sample={:>6} | actual_sample_rate={}Hz (target 44100)",
                now_us / 1_000_000,
                LAST_SAMPLE.load(Ordering::Relaxed),
                actual_rate,
            );
        }
    }
}
