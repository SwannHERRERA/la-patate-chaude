use std::time::Duration;

pub const IP: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 7878;
pub const LOG_LEVEL: &'static str = "debug";
pub const TIMEOUT: Duration = Duration::from_secs(10);
pub const ROUND: usize = 3;