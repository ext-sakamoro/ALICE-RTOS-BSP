#![no_std]
#![doc = "M5Stack Cardputer BSP — ESP32-S3-PICO-1, 8MB Flash + 8MB PSRAM, 1.14\" ST7789 LCD 240x135, 56キーキーボード, NS4168 スピーカー, PDM マイク, microSD, IR"]

/// M5Stack Cardputer ピンアサイン
pub mod pinout {
    /// LCD (ST7789, 240x135 横置き) — SPI
    pub mod lcd {
        pub const SCLK: u8 = 40;
        pub const MOSI: u8 = 41;
        pub const CS: u8 = 37;
        pub const DC: u8 = 34;
        pub const RST: u8 = 33;
        pub const BL: u8 = 38;
        pub const WIDTH: u16 = 240;
        pub const HEIGHT: u16 = 135;
    }

    /// PDM マイク (内蔵)
    pub mod mic {
        pub const DATA: u8 = 46;
        pub const CLK: u8 = 43;
    }

    /// I2S スピーカー (NS4168 D-class amp)
    pub mod speaker {
        pub const BCK: u8 = 41; // LCD MOSI と共有
        pub const LRCLK: u8 = 43;
        pub const DATA: u8 = 42;
    }

    /// SD カード (SPI モード)
    pub mod sd {
        pub const SCLK: u8 = 40; // LCD SCLK と共有
        pub const MOSI: u8 = 14;
        pub const MISO: u8 = 39;
        pub const CS: u8 = 12;
    }

    /// 赤外線送信
    pub const IR_TX: u8 = 44;

    /// HAT/外部 Grove (4-pin: G+, G-, G13, G15)
    pub mod hat {
        pub const G1: u8 = 13;
        pub const G2: u8 = 15;
    }

    /// 56キー キーボード マトリクス (8 出力 × 7 入力)
    ///
    /// 出力 (走査): KB_OUT0..KB_OUT7
    /// 入力 (読み): KB_IN0..KB_IN6
    ///
    /// 実際の配線は M5Stack 公式 datasheet 参照。代表ピンのみ記載。
    pub mod keyboard {
        pub const KB_OUT: [u8; 8] = [8, 9, 11, 13, 15, 3, 4, 5];
        pub const KB_IN: [u8; 7] = [6, 7, 1, 2, 35, 36, 37];
    }
}

/// ESP32-S3 既定動作クロック (Hz)
pub const CPU_HZ: u32 = 240_000_000;
