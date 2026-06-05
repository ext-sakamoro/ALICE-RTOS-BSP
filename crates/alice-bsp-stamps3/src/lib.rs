#![no_std]
#![doc = "M5StampS3 BSP — ESP32-S3-FN8, 8MB Flash, no LCD, no IMU, 21 GPIO 露出"]

/// M5StampS3 ピンアサイン (露出ピン中心)
pub mod pinout {
    /// 内蔵 WS2812 RGB LED (基板上の SK6812)
    pub const WS2812_LED: u8 = 21;
    /// 内蔵ユーザーボタン (G0/Boot 共用)
    pub const BUTTON:     u8 = 0;

    /// 露出 GPIO 群 (側面 21ピン)
    pub mod io {
        pub const G1:  u8 = 1;
        pub const G2:  u8 = 2;
        pub const G3:  u8 = 3;
        pub const G4:  u8 = 4;
        pub const G5:  u8 = 5;
        pub const G6:  u8 = 6;
        pub const G7:  u8 = 7;
        pub const G8:  u8 = 8;
        pub const G9:  u8 = 9;
        pub const G10: u8 = 10;
        pub const G11: u8 = 11;
        pub const G12: u8 = 12;
        pub const G13: u8 = 13;
        pub const G14: u8 = 14;
        pub const G15: u8 = 15;
        pub const G39: u8 = 39;
        pub const G40: u8 = 40;
        pub const G41: u8 = 41;
        pub const G42: u8 = 42;
        pub const G43: u8 = 43;
        pub const G44: u8 = 44;
    }
}

/// ESP32-S3 既定動作クロック (Hz)
pub const CPU_HZ: u32 = 240_000_000;
