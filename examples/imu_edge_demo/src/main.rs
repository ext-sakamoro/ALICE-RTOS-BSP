#![no_std]
#![no_main]

//! IMU Edge demo — ALICE-RTOS の `edge` feature 正規版
//!
//! `alice_rtos::edge_tasks` モジュールが提供する 1kHz 用 task テンプレート
//! `edge_task` を使い、100Hz 加速度サンプリング + 線形傾き分類器を実装。
//!
//! M5Stack AtomS3 ベース機は IMU 非搭載 (R 版のみ BMI270 内蔵)。
//! 起動時に I2C scan を行い、応答デバイスがなければその旨を表示する設計。

use alice_bsp_atoms3::{CPU_HZ, MPU6886_I2C_ADDR};
use alice_rtos::{edge_tasks, Kernel};
use core::cell::RefCell;
use core::sync::atomic::{AtomicI32, AtomicU8, Ordering};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config as I2cConfig, I2c},
    main,
    time::Rate,
    Blocking,
};
use esp_println::println;

esp_app_desc!();

static I2C_BUS: Mutex<RefCell<Option<I2c<'static, Blocking>>>> = Mutex::new(RefCell::new(None));
static AX_MG: AtomicI32 = AtomicI32::new(0);
static AY_MG: AtomicI32 = AtomicI32::new(0);
static AZ_MG: AtomicI32 = AtomicI32::new(0);
static TILT: AtomicU8 = AtomicU8::new(0);

const TILT_LABELS: [&str; 6] = [
    "FLAT",
    "LEFT",
    "RIGHT",
    "FORWARD",
    "BACKWARD",
    "UPSIDE_DOWN",
];

fn mpu_init(i2c: &mut I2c<'static, Blocking>) -> bool {
    let mut who = [0u8; 1];
    if i2c.write_read(MPU6886_I2C_ADDR, &[0x75], &mut who).is_err() {
        return false;
    }
    if who[0] != 0x19 {
        println!(
            "MPU6886 WHO_AM_I unexpected: 0x{:02x} (expected 0x19)",
            who[0]
        );
        return false;
    }
    let _ = i2c.write(MPU6886_I2C_ADDR, &[0x6B, 0x80]);
    Delay::new().delay_millis(100);
    let _ = i2c.write(MPU6886_I2C_ADDR, &[0x6B, 0x01]);
    let _ = i2c.write(MPU6886_I2C_ADDR, &[0x1C, 0x00]); // ±2g
    Delay::new().delay_millis(10);
    true
}

fn read_accel(i2c: &mut I2c<'static, Blocking>) -> Option<(i16, i16, i16)> {
    let mut buf = [0u8; 6];
    i2c.write_read(MPU6886_I2C_ADDR, &[0x3B], &mut buf).ok()?;
    Some((
        i16::from_be_bytes([buf[0], buf[1]]),
        i16::from_be_bytes([buf[2], buf[3]]),
        i16::from_be_bytes([buf[4], buf[5]]),
    ))
}

fn task_edge_classify(_scratch: &mut [u8]) {
    critical_section::with(|cs| {
        if let Some(i2c) = I2C_BUS.borrow(cs).borrow_mut().as_mut() {
            if let Some((ax, ay, az)) = read_accel(i2c) {
                let to_mg = |v: i16| -> i32 { (v as i32) * 1000 / 16384 };
                let ax_mg = to_mg(ax);
                let ay_mg = to_mg(ay);
                let az_mg = to_mg(az);
                AX_MG.store(ax_mg, Ordering::Relaxed);
                AY_MG.store(ay_mg, Ordering::Relaxed);
                AZ_MG.store(az_mg, Ordering::Relaxed);

                let abs = |v: i32| if v < 0 { -v } else { v };
                let label: u8 = if az_mg < -700 {
                    5
                } else if az_mg > 800 && abs(ax_mg) < 400 && abs(ay_mg) < 400 {
                    0
                } else if abs(ax_mg) > abs(ay_mg) {
                    if ax_mg > 0 {
                        2
                    } else {
                        1
                    }
                } else if ay_mg > 0 {
                    3
                } else {
                    4
                };
                TILT.store(label, Ordering::Relaxed);
            }
        }
    });
}

#[main]
fn main() -> ! {
    let p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS IMU Edge demo @ AtomS3 (ALICE-RTOS edge feature) ===");
    println!(
        " EDGE_PERIOD_US={} ({}Hz default), EDGE_WCET_US={}, PRIORITY=NORMAL",
        edge_tasks::EDGE_PERIOD_US,
        1_000_000 / edge_tasks::EDGE_PERIOD_US,
        edge_tasks::EDGE_WCET_US,
    );
    let util = edge_tasks::edge_utilization(edge_tasks::EDGE_PERIOD_US, edge_tasks::EDGE_WCET_US);
    println!(" Default CPU utilization estimate: {:.2}%", util * 100.0);

    let i2c = I2c::new(
        p.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .expect("I2C init")
    .with_sda(p.GPIO38)
    .with_scl(p.GPIO39);

    critical_section::with(|cs| {
        *I2C_BUS.borrow(cs).borrow_mut() = Some(i2c);
        if let Some(i2c) = I2C_BUS.borrow(cs).borrow_mut().as_mut() {
            println!("I2C scan (SDA=GPIO38, SCL=GPIO39):");
            let mut found = 0;
            for addr in 0x08u8..=0x77 {
                let mut probe = [0u8; 1];
                if i2c.write_read(addr, &[0x00], &mut probe).is_ok() {
                    println!("  responder at 0x{:02x}", addr);
                    found += 1;
                }
            }
            if found == 0 {
                println!("  (no devices) — このAtomS3はIMU非搭載 (R版のみ搭載)");
            }
            if !mpu_init(i2c) {
                println!("MPU6886 init FAILED (期待動作: AtomS3 ベース機はIMU非搭載)");
            } else {
                println!("MPU6886 ready (WHO_AM_I=0x19)");
            }
        }
    });

    let mut kernel = Kernel::new(CPU_HZ);
    // 100Hz サンプリングなので edge_task_default (1kHz) ではなく custom 周期
    let edge = edge_tasks::edge_task(task_edge_classify, 10_000, 200);
    kernel.scheduler.register(edge).expect("edge task register");

    println!("100Hz accel sampling + edge tilt classifier started");

    let mut last_log_us: u64 = 0;
    loop {
        kernel.tick(1_000);
        delay.delay_micros(1_000);
        let now_us = kernel.total_ticks * 1_000;
        if now_us - last_log_us >= 500_000 {
            last_log_us = now_us;
            let t = TILT.load(Ordering::Relaxed) as usize;
            println!(
                "[t={:>5}ms] ax={:>5} ay={:>5} az={:>5} mg | tilt={}",
                now_us / 1_000,
                AX_MG.load(Ordering::Relaxed),
                AY_MG.load(Ordering::Relaxed),
                AZ_MG.load(Ordering::Relaxed),
                TILT_LABELS[t.min(5)],
            );
        }
    }
}
