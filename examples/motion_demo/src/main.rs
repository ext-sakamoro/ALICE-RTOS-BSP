#![no_std]
#![no_main]

//! Motion demo (ALICE-Motion 風 軌道生成)
//!
//! 外付けステッパドライバ (A4988 / DRV8825 / TMC2209) を AtomS3 の Grove ポートに接続:
//! - STEP: G1 (GPIO 2)
//! - DIR : G2 (GPIO 1)
//!
//! 10kHz で台形加減速軌道を計算し、step pulse を発行。
//! 本来の ALICE-Motion は NURBS 軌道 + 4次補間 + ジャーク制御をやるが、ここでは
//! 簡略化して台形プロファイル + ステップ送出のみ。
//!
//! ビルド検証のみ。実機テストにはステッパドライバ + モータが必要。

use alice_bsp_atoms3::{CPU_HZ, pinout::grove};
use alice_rtos::{Kernel, TaskPriority};
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_app_desc!();

const TICK_HZ: u32 = 10_000;
const TICK_US: u32 = 1_000_000 / TICK_HZ;

/// 台形プロファイル: ramp_up → cruise → ramp_down (各 1万ステップ)
const STEPS_RAMP:  i32 = 10_000;
const STEPS_CRUISE: i32 = 10_000;
const STEPS_TOTAL: i32 = STEPS_RAMP * 2 + STEPS_CRUISE;

static PROFILE_TICK: AtomicU32 = AtomicU32::new(0);
static STEPS_DONE:   AtomicI32 = AtomicI32::new(0);
/// 速度 (mステップ/sec, Q16 固定小数 — 本来は f32 だが no_FPU 想定で固定小数)
static VELOCITY_MILLI: AtomicI32 = AtomicI32::new(0);

/// 10kHz ステッパ軌道タスク
fn task_motion_step(_scratch: &mut [u8]) {
    let tick = PROFILE_TICK.fetch_add(1, Ordering::Relaxed) as i32;
    let phase = tick % STEPS_TOTAL;
    // 線形台形 (ジャーク制御は省略)
    let v = if phase < STEPS_RAMP {
        // 加速: 0 → 100_000 (= 100 steps/sec)
        (phase * 100_000) / STEPS_RAMP
    } else if phase < STEPS_RAMP + STEPS_CRUISE {
        100_000
    } else {
        let down_phase = phase - STEPS_RAMP - STEPS_CRUISE;
        100_000 - (down_phase * 100_000) / STEPS_RAMP
    };
    VELOCITY_MILLI.store(v, Ordering::Relaxed);

    // 簡易 step 発行ロジック (実機では GPIO toggle)
    STEPS_DONE.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let _p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS Motion demo @ AtomS3 (no stepper attached) ===");
    println!(" Pinout: STEP=Grove G1 (GPIO {}), DIR=Grove G2 (GPIO {})",
             grove::G1, grove::G2);
    println!(" Profile: ramp {} steps, cruise {} steps, ramp-down {} steps",
             STEPS_RAMP, STEPS_CRUISE, STEPS_RAMP);

    let mut kernel = Kernel::new(CPU_HZ);
    kernel
        .add_task(b"motion", task_motion_step, TaskPriority::CRITICAL,
                  TICK_US, 20)
        .expect("motion task");

    println!("10kHz trajectory scheduler started");

    let mut last_log_us: u64 = 0;
    loop {
        kernel.tick(50);
        delay.delay_micros(50);
        let now_us = kernel.total_ticks * 50;
        if now_us - last_log_us >= 500_000 {
            last_log_us = now_us;
            println!("[t={:>5}ms] steps={} v={}m_steps/s",
                     now_us / 1_000,
                     STEPS_DONE.load(Ordering::Relaxed),
                     VELOCITY_MILLI.load(Ordering::Relaxed));
        }
    }
}
