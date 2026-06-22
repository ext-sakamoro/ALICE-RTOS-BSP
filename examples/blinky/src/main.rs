#![no_std]
#![no_main]

use alice_bsp_atoms3::CPU_HZ;
use alice_rtos::{Kernel, TaskPriority};
use core::sync::atomic::{AtomicU32, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_app_desc!();

static TICK_COUNT: AtomicU32 = AtomicU32::new(0);
static HEARTBEAT: AtomicU32 = AtomicU32::new(0);

fn task_tick(_scratch: &mut [u8]) {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}

fn task_heartbeat(_scratch: &mut [u8]) {
    HEARTBEAT.fetch_add(1, Ordering::Relaxed);
}

#[main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("============================================");
    println!(" ALICE-RTOS-BSP / AtomS3 (ESP32-S3-PICO-1)");
    println!(" Math-First RTOS booting...");
    println!("============================================");

    let mut kernel = Kernel::new(CPU_HZ);

    let _ = kernel
        .add_task(b"tick_10hz", task_tick, TaskPriority::NORMAL, 100_000, 50)
        .expect("tick_10hz registration");
    let _ = kernel
        .add_task(b"hb_1hz", task_heartbeat, TaskPriority::LOW, 1_000_000, 50)
        .expect("hb_1hz registration");

    println!("Tasks registered. Entering scheduler loop @ 1ms tick.");

    let mut last_report_us: u64 = 0;
    loop {
        kernel.tick(1_000);
        delay.delay_micros(1_000);

        let now_us = kernel.total_ticks * 1_000;
        if now_us - last_report_us >= 5_000_000 {
            last_report_us = now_us;
            println!(
                "[t={:>4}s] ticks={} hb={}",
                now_us / 1_000_000,
                TICK_COUNT.load(Ordering::Relaxed),
                HEARTBEAT.load(Ordering::Relaxed),
            );
        }
    }
}
