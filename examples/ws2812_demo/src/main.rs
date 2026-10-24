#![no_std]
#![no_main]

use alice_bsp_atoms3::CPU_HZ;
use alice_rtos::{Kernel, TaskPriority};
use core::sync::atomic::{AtomicU8, Ordering};
use esp_backtrace as _;
use esp_bootloader_esp_idf::esp_app_desc;
use esp_hal::{
    delay::Delay,
    gpio::Level,
    main,
    rmt::{PulseCode, Rmt, TxChannelConfig, TxChannelCreator},
    time::Rate,
};
use esp_println::println;
use smart_leds::{brightness, gamma, RGB8};

esp_app_desc!();

static HUE: AtomicU8 = AtomicU8::new(0);

fn task_hue_bump(_scratch: &mut [u8]) {
    HUE.fetch_add(2, Ordering::Relaxed);
}

fn hsv_to_rgb(h: u8) -> RGB8 {
    let region = h / 43;
    let remainder = h.wrapping_sub(region.wrapping_mul(43)).wrapping_mul(6);
    match region {
        0 => RGB8 {
            r: 255,
            g: remainder,
            b: 0,
        },
        1 => RGB8 {
            r: 255 - remainder,
            g: 255,
            b: 0,
        },
        2 => RGB8 {
            r: 0,
            g: 255,
            b: remainder,
        },
        3 => RGB8 {
            r: 0,
            g: 255 - remainder,
            b: 255,
        },
        4 => RGB8 {
            r: remainder,
            g: 0,
            b: 255,
        },
        _ => RGB8 {
            r: 255,
            g: 0,
            b: 255 - remainder,
        },
    }
}

/// WS2812 bit timing in RMT ticks at 80 MHz with clock divider 1 (12.5 ns per tick):
/// a 0 bit is 0.40 us high then 0.85 us low, a 1 bit 0.80 us high then 0.45 us low
const T0H: u16 = 32;
const T0L: u16 = 68;
const T1H: u16 = 64;
const T1L: u16 = 36;

/// The RMT pulse codes of one LED in the WS2812's GRB order, most significant bit first, then the
/// end marker (the idle low level after it is the latch)
fn led_pulses(c: RGB8) -> [PulseCode; 25] {
    let mut out = [PulseCode::end_marker(); 25];
    for (i, byte) in [c.g, c.r, c.b].into_iter().enumerate() {
        for bit in 0..8 {
            out[i * 8 + bit] = if byte & (0x80 >> bit) != 0 {
                PulseCode::new(Level::High, T1H, Level::Low, T1L)
            } else {
                PulseCode::new(Level::High, T0H, Level::Low, T0L)
            };
        }
    }
    out
}

#[main]
fn main() -> ! {
    let p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!();
    println!("=== ALICE-RTOS WS2812 demo @ AtomS3 ===");

    let freq = Rate::from_mhz(80);
    let rmt = Rmt::new(p.RMT, freq).expect("RMT init failed");
    let mut led = Some(
        rmt.channel0
            .configure_tx(
                &TxChannelConfig::default()
                    .with_clk_divider(1)
                    .with_idle_output_level(Level::Low)
                    .with_idle_output(true)
                    .with_memsize(2),
            )
            .expect("LED init")
            .with_pin(p.GPIO35),
    );

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
            let shown = brightness(gamma([color].iter().copied()), 30)
                .next()
                .unwrap_or_default();
            let pulses = led_pulses(shown);
            if let Some(ch) = led.take() {
                led = Some(match ch.transmit(&pulses) {
                    Ok(tx) => tx.wait().unwrap_or_else(|(_, ch)| ch),
                    Err((_, ch)) => ch,
                });
            }
        }

        if now_us - last_log_us >= 5_000_000 {
            last_log_us = now_us;
            println!(
                "[t={}s] hue={}",
                now_us / 1_000_000,
                HUE.load(Ordering::Relaxed)
            );
        }
    }
}
