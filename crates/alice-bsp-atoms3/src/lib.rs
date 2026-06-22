#![no_std]
#![doc = "M5Stack AtomS3 BSP — ESP32-S3-PICO-1, 8MB Flash + 8MB PSRAM, 0.85\" LCD, WS2812 LED, MPU6886 IMU"]

/// M5Stack AtomS3 ピンアサイン
pub mod pinout {
    pub const WS2812_LED: u8 = 35;
    pub const BUTTON: u8 = 41;

    /// 内部 I2C0 (AtomS3R / 外付け IMU 用)。
    /// **注意**: ベース AtomS3 (LCD のみ) は IMU 非搭載。AtomS3R は BMI270。
    pub mod imu_i2c {
        pub const SDA: u8 = 38;
        pub const SCL: u8 = 39;
    }

    pub mod lcd {
        pub const CS: u8 = 15;
        pub const DC: u8 = 33;
        pub const RST: u8 = 34;
        pub const SCLK: u8 = 17;
        pub const MOSI: u8 = 21;
        pub const BL: u8 = 16;
        pub const WIDTH: u16 = 128;
        pub const HEIGHT: u16 = 128;
    }

    pub mod grove {
        pub const G1: u8 = 2;
        pub const G2: u8 = 1;
    }
}

/// ESP32-S3 既定動作クロック (Hz)
pub const CPU_HZ: u32 = 240_000_000;

/// MPU6886 I2C 7-bit アドレス (Grove 外付け or 旧 ATOM-Matrix 用)
pub const MPU6886_I2C_ADDR: u8 = 0x68;

/// BMI270 I2C 7-bit アドレス (AtomS3R 内蔵)
pub const BMI270_I2C_ADDR: u8 = 0x69;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinout_constants_are_distinct() {
        assert_ne!(pinout::WS2812_LED, pinout::BUTTON);
        assert_ne!(pinout::imu_i2c::SDA, pinout::imu_i2c::SCL);
    }

    #[test]
    fn cpu_hz_is_240mhz() {
        assert_eq!(CPU_HZ, 240_000_000);
    }
}
