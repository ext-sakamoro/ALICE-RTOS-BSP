#![no_std]
#![doc = "M5Stack CoreS3 BSP — ESP32-S3, 16MB Flash + 8MB PSRAM, AXP2101 PMIC, BMI270 IMU, 320x240 タッチLCD (ILI9342C+FT6336U), ES8311 codec"]

/// M5Stack CoreS3 ピンアサイン
pub mod pinout {
    /// 内部 I2C0 バス (AXP2101 + AW9523 + BMI270 + FT6336U タッチ + ES8311 codec が全部ぶら下がる)
    pub mod i2c0 {
        pub const SDA: u8 = 12;
        pub const SCL: u8 = 11;
    }

    /// LCD (ILI9342C, 320x240) — SPI
    pub mod lcd {
        pub const MOSI: u8 = 37;
        pub const MISO: u8 = 35;
        pub const SCLK: u8 = 36;
        pub const CS: u8 = 3;
        pub const DC: u8 = 35;
        pub const WIDTH: u16 = 320;
        pub const HEIGHT: u16 = 240;
        // RST / BL は AXP2101 (LDO) 経由、GPIO 直接制御なし
    }

    /// タッチ (FT6336U) — I2C0 共有 + 割込ピン
    pub mod touch {
        pub const INT: u8 = 21;
    }

    /// SD カード (SDMMC 4-bit)
    pub mod sd {
        pub const CLK: u8 = 36; // SD_SCLK 共有
        pub const CMD: u8 = 35;
        pub const D0: u8 = 37;
        pub const D1: u8 = 38;
        pub const D2: u8 = 33;
        pub const D3: u8 = 34;
    }

    /// I2S (ES8311 codec 用、speaker + mic 兼用)
    pub mod i2s {
        pub const BCK: u8 = 34;
        pub const WS: u8 = 33;
        pub const SDO: u8 = 13; // speaker out
        pub const SDI: u8 = 14; // mic in (ES7210 経由)
        pub const MCLK: u8 = 0;
    }

    /// USB-C OTG (内部 PHY)
    pub mod usb {
        pub const DM: u8 = 19;
        pub const DP: u8 = 20;
    }

    /// Grove ポート (3個)
    pub mod grove {
        pub mod port_a {
            pub const G1: u8 = 1; // SDA (I2C 兼用)
            pub const G2: u8 = 2; // SCL
        }
        pub mod port_b {
            pub const G1: u8 = 8;
            pub const G2: u8 = 9;
        }
        pub mod port_c {
            pub const G1: u8 = 18; // RX (UART2)
            pub const G2: u8 = 17; // TX
        }
    }
}

/// ESP32-S3 既定動作クロック (Hz)
pub const CPU_HZ: u32 = 240_000_000;

/// AXP2101 PMIC I2C 7-bit アドレス
pub const AXP2101_I2C_ADDR: u8 = 0x34;

/// AW9523 GPIO エキスパンダ I2C 7-bit アドレス (LED / 各種制御用)
pub const AW9523_I2C_ADDR: u8 = 0x58;

/// BMI270 6軸 IMU I2C 7-bit アドレス
pub const BMI270_I2C_ADDR: u8 = 0x68;

/// FT6336U タッチコントローラ I2C 7-bit アドレス
pub const FT6336U_I2C_ADDR: u8 = 0x38;

/// ES8311 オーディオ codec I2C 7-bit アドレス
pub const ES8311_I2C_ADDR: u8 = 0x18;
