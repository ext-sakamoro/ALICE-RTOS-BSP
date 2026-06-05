#![no_std]
#![doc = "M5Atom Lite BSP — ESP32-PICO-D4 (Xtensa LX6 dual), 4MB Flash, 内蔵 WS2812"]

/// M5Atom Lite ピンアサイン
pub mod pinout {
    /// 内蔵 WS2812 RGB LED (1個)
    pub const WS2812_LED: u8 = 27;
    /// 内蔵ユーザーボタン
    pub const BUTTON:     u8 = 39;
    /// 赤外線LED
    pub const IR_LED:     u8 = 12;

    /// Grove Port 1 (4-pin: 5V/G/G32/G26)
    pub mod grove {
        pub const G1: u8 = 32;
        pub const G2: u8 = 26;
    }

    /// 底面 6-pin ヘッダ
    pub mod io {
        pub const G19: u8 = 19;
        pub const G21: u8 = 21;
        pub const G22: u8 = 22;
        pub const G23: u8 = 23;
        pub const G25: u8 = 25;
        pub const G33: u8 = 33;
    }
}

/// ESP32 既定動作クロック (Hz)
pub const CPU_HZ: u32 = 240_000_000;
