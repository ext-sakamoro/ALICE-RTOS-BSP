#![no_std]
#![no_main]

use alice_bsp_atoms3::CPU_HZ;
use alice_rtos::{Kernel, TaskPriority};
use core::sync::atomic::{AtomicU8, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{delay::Delay, main, rmt::Rmt, time::Rate};
use esp_hal_smartled::{buffer_size, color_order, RmtSmartLeds, Ws2812Timing};
use esp_println::println;
use smart_leds::{brightness, gamma, SmartLedsWrite, RGB8};

esp_app_desc!();

static HUE: AtomicU8 = AtomicU8::new(0);

fn task_hue_bump(_scratch: &mut [u8]) {
    HUE.fetch_add(2, Ordering::Relaxed);
}

fn hsv_to_rgb(h: u8) -> RGB8 {
    let region = h / 43;
    let remainder = h.wrapping_sub(region.wrapping_mul(43)).wrapping_mul(6);
    match region {
        0 => RGB8 { r: 255, g: remainder, b: 0 },
        1 => RGB8 { r: 255 - remainder, g: 255, b: 0 },
        2 => RGB8 { r: 0, g: 255, b: remainder },
        3 => RGB8 { r: 0, g: 255 - remainder, b: 255 },
        4 => RGB8 { r: remainder, g: 0, b: 255 },
        _ => RGB8 { r: 255, g: 0, b: 255 - remainder },
    }
}

type LedColor = RGB8;

#[main]
fn main() -> ! {
    let p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS WS2812 demo @ AtomS3 ===");

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(p.RMT, freq).expect("RMT init failed");
    let mut led = RmtSmartLeds::<
        { buffer_size::<LedColor>(1) },
        _,
        LedColor,
        color_order::Grb,
        Ws2812Timing,
    >::new_with_memsize(rmt.channel0, p.GPIO35, 2)
        .expect("LED init");

    let mut kernel = Kernel::new(CPU_HZ);
    kernel
        .add_task(b"hue_bump", task_hue_bump, TaskPriority::NORMAL, 50_000, 50)
        .expect("task register");

    println!("RGB hue loop running, 20Hz hue task + 33Hz render");

    let mut last_render_us: u64 = 0;
    let mut last_log_us: u64 = 0;
    loop {
        kernel.tick(1_000);
        delay.delay_micros(1_000);

        let now_us = kernel.total_ticks * 1_000;

        if now_us - last_render_us >= 30_000 {
            last_render_us = now_us;
            let h = HUE.load(Ordering::Relaxed);
            let color = hsv_to_rgb(h);
            let _ = led.write(brightness(gamma([color].iter().copied()), 30));
        }

        if now_us - last_log_us >= 5_000_000 {
            last_log_us = now_us;
            println!("[t={}s] hue={}", now_us / 1_000_000, HUE.load(Ordering::Relaxed));
        }
    }
}
