#![no_std]
#![no_main]

//! Motion demo — ALICE-RTOS の `motion` feature 正規版
//!
//! `alice_rtos::motion_tasks` モジュールが提供する 50kHz 用 stepper task テンプレート
//! `motion_task_stepper` を使い、CRITICAL 優先度 + 20µs 周期で step 生成タスクを
//! スケジュール。
//!
//! 外付けステッパドライバ (A4988 / DRV8825 / TMC2209) を AtomS3 の Grove ポートに接続:
//! - STEP: G1 (GPIO 2)
//! - DIR : G2 (GPIO 1)
//!
//! 台形プロファイル + step 送出のみの簡略版 (本物の ALICE-Motion は NURBS + ジャーク制御)。

use alice_bsp_atoms3::{pinout::grove, CPU_HZ};
use alice_rtos::{motion_tasks, Kernel};
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_app_desc!();

const STEPS_RAMP: i32 = 10_000;
const STEPS_CRUISE: i32 = 10_000;
const STEPS_TOTAL: i32 = STEPS_RAMP * 2 + STEPS_CRUISE;

static PROFILE_TICK: AtomicU32 = AtomicU32::new(0);
static STEPS_DONE: AtomicI32 = AtomicI32::new(0);
static VELOCITY_MILLI: AtomicI32 = AtomicI32::new(0);
static STEPS_PER_LOG: AtomicU32 = AtomicU32::new(0);

/// 50kHz ステッパ軌道タスク (motion_task_stepper 既定周期 20µs)
fn task_motion_step(_scratch: &mut [u8]) {
    let tick = PROFILE_TICK.fetch_add(1, Ordering::Relaxed) as i32;
    let phase = tick % STEPS_TOTAL;
    let v = if phase < STEPS_RAMP {
        (phase * 100_000) / STEPS_RAMP
    } else if phase < STEPS_RAMP + STEPS_CRUISE {
        100_000
    } else {
        let down_phase = phase - STEPS_RAMP - STEPS_CRUISE;
        100_000 - (down_phase * 100_000) / STEPS_RAMP
    };
    VELOCITY_MILLI.store(v, Ordering::Relaxed);
    STEPS_DONE.fetch_add(1, Ordering::Relaxed);
    STEPS_PER_LOG.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let _p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS Motion demo @ AtomS3 (ALICE-RTOS motion feature) ===");
    println!(
        " Pinout: STEP=Grove G1 (GPIO {}), DIR=Grove G2 (GPIO {})",
        grove::G1,
        grove::G2,
    );
    println!(
        " MOTION_STEPPER_PERIOD_US={} ({}Hz), CRITICAL priority",
        motion_tasks::MOTION_STEPPER_PERIOD_US,
        1_000_000 / motion_tasks::MOTION_STEPPER_PERIOD_US,
    );
    println!(
        " 3-DOF capacity @ 10kHz: max_dof={}",
        motion_tasks::max_dof(
            motion_tasks::MOTION_PERIOD_US,
            motion_tasks::MOTION_WCET_US / 3
        ),
    );
    println!(
        " Profile: ramp={} steps, cruise={} steps, ramp-down={} steps (total {})",
        STEPS_RAMP, STEPS_CRUISE, STEPS_RAMP, STEPS_TOTAL,
    );

    let mut kernel = Kernel::new(CPU_HZ);
    let stepper = motion_tasks::motion_task_stepper(task_motion_step, 10);
    kernel
        .scheduler
        .register(stepper)
        .expect("stepper register");

    println!("Stepper trajectory scheduler started (no motor attached — sample output only)");

    let mut last_log_us: u64 = 0;
    loop {
        kernel.tick(10);
        delay.delay_micros(10);
        let now_us = kernel.total_ticks * 10;
        if now_us - last_log_us >= 500_000 {
            last_log_us = now_us;
            let steps_recent = STEPS_PER_LOG.swap(0, Ordering::Relaxed);
            println!(
                "[t={:>5}ms] steps={} v={}m_steps/s | rate={}steps/0.5s (target 25000)",
                now_us / 1_000,
                STEPS_DONE.load(Ordering::Relaxed),
                VELOCITY_MILLI.load(Ordering::Relaxed),
                steps_recent,
            );
        }
    }
}
