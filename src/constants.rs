use uuid::Uuid;

// Nordic UART Service
pub const UUID_WRITE: Uuid = Uuid::from_u128(0x6e400002b5a3f393e0a9e50e24dcca9e);
pub const UUID_READ: Uuid = Uuid::from_u128(0x6e400003b5a3f393e0a9e50e24dcca9e);

pub const CMD_START_HEART_RATE: u8 = 0x69;
pub const CMD_STOP_HEART_RATE: u8 = 0x6A;

pub const SCAN_REFRESH_SECS: u64 = 3;
pub const KEEP_ALIVE_SECS: u64 = 60; // some models stop streaming without a periodic re-trigger
pub const BPM_RETRIGGER_SECS: u64 = 5; // re-send START if no valid BPM received for this long
pub const BLE_CHANNEL_BUFFER: usize = 100;

// Packet format: [cmd (1 B)][payload (14 B, zero-padded)][checksum (1 B)]
pub const PACKET_SIZE: usize = 16;

// Ring reports RR interval in ms; BPM = 60000 / rr_ms
pub const RR_TO_BPM_MS: u32 = 60_000;
pub const MIN_RR_INTERVAL_MS: u32 = 300; // below this → BPM > 200, discard

// Ring sends 0xEE when it loses skin contact — filter it out along with other non-human values
pub const MIN_VALID_BPM: u8 = 30;
pub const MAX_VALID_BPM: u8 = 200;

// BPM zone upper bounds (Alarm = everything above FAST_MAX)
pub const BPM_ZONE_CALM_MAX: u8 = 64;
pub const BPM_ZONE_NORMAL_MAX: u8 = 80;
pub const BPM_ZONE_HIGH_MAX: u8 = 100;
pub const BPM_ZONE_FAST_MAX: u8 = 130;

// Lower bounds derived from upper bounds, used in match range patterns to avoid overlapping arms.
pub const BPM_ZONE_NORMAL_MIN: u8 = BPM_ZONE_CALM_MAX + 1;
pub const BPM_ZONE_HIGH_MIN: u8 = BPM_ZONE_NORMAL_MAX + 1;
pub const BPM_ZONE_FAST_MIN: u8 = BPM_ZONE_HIGH_MAX + 1;

pub const WINDOW_WIDTH: f32 = 440.0;
pub const WINDOW_HEIGHT: f32 = 400.0;

// Embedded at compile time by build.rs (Vite build output)
pub const OVERLAY_HTML: &str = include_str!("../overlay/dist/index.html");

// Bind port
pub const PORT: u16 = 9000;
