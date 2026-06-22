#![no_std]
#![doc = "M5StickC Plus2 BSP — ESP32-PICO-V3-02 (Xtensa LX6 dual), 8MB Flash, 1.14\" ST7789v2 LCD 135x240, MPU6886 IMU, IR LED, ブザー, RTC BM8563"]

/// M5StickC Plus2 ピンアサイン
pub mod pinout {
    /// LCD (ST7789v2, 135x240) — SPI
    pub mod lcd {
        pub const SCLK: u8 = 13;
        pub const MOSI: u8 = 15;
        pub const CS: u8 = 5;
        pub const DC: u8 = 14;
        pub const RST: u8 = 12;
        pub const BL: u8 = 27;
        pub const WIDTH: u16 = 135;
        pub const HEIGHT: u16 = 240;
    }

    /// 内蔵 I2C0 (MPU6886 IMU + BM8563 RTC)
    pub mod imu_i2c {
        pub const SDA: u8 = 21;
        pub const SCL: u8 = 22;
    }

    /// ボタン (3個: A=Front, B=Right Side, PWR=長押し電源OFF)
    pub mod button {
        pub const A: u8 = 37;
        pub const B: u8 = 39;
        pub const PWR: u8 = 35;
    }

    /// 内蔵 IR LED
    pub const IR_LED: u8 = 19;

    /// 内蔵ブザー (PWM)
    pub const BUZZER: u8 = 2;

    /// 内蔵 LED (赤)
    pub const LED: u8 = 19; // IR LED と共用 (ハード上)

    /// 内蔵 PDM マイク (SPM1423)
    pub mod mic {
        pub const DATA: u8 = 34;
        pub const CLK: u8 = 0;
    }

    /// Grove (HY2.0-4P, I2C/UART/GPIO 兼用)
    pub mod grove {
        pub const G1: u8 = 32; // SDA / RX / GPIO
        pub const G2: u8 = 33; // SCL / TX / GPIO
    }

    /// 8-pin HAT / M-Bus (GPIO 0, 25, 26, 36)
    pub mod hat {
        pub const G0: u8 = 0;
        pub const G25: u8 = 25;
        pub const G26: u8 = 26;
        pub const G36: u8 = 36;
    }
}

/// ESP32 既定動作クロック (Hz)
pub const CPU_HZ: u32 = 240_000_000;

/// MPU6886 6軸 IMU I2C 7-bit アドレス
pub const MPU6886_I2C_ADDR: u8 = 0x68;

/// BM8563 RTC I2C 7-bit アドレス
pub const BM8563_I2C_ADDR: u8 = 0x51;
