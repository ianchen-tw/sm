use ratatui::style::Color;

#[derive(Copy, Clone)]
pub struct PingInfo {
    pub emoji: &'static str,
    pub color: Color,
    pub description: &'static str,
}

impl PingInfo {
    pub fn format_with_latency(&self, latency: u32) -> String {
        format!("{latency}ms")
    }
}

/// Generate a legend string showing all ping categories and their thresholds.
pub fn get_ping_legend() -> String {
    "Ping: 🟢 Good (<100ms) | 🟡 OK (<200ms) | 🟠 Slow (<350ms) | 🔴 Poor (>350ms)".to_string()
}

/// Get the appropriate ping info for a given latency value.
pub fn get_ping_info_for_latency(latency: u32) -> PingInfo {
    match latency {
        0..=100 => PingInfo {
            emoji: "🟢",
            color: Color::Green,
            description: "Good",
        },
        101..=200 => PingInfo {
            emoji: "🟡",
            color: Color::Yellow,
            description: "OK",
        },
        201..=350 => PingInfo {
            emoji: "🟠",
            color: Color::LightRed,
            description: "Slow",
        },
        _ => PingInfo {
            emoji: "🔴",
            color: Color::Red,
            description: "Poor",
        },
    }
}

/// Get ping info for connection failures
pub fn get_ping_failure_info() -> PingInfo {
    PingInfo {
        emoji: "❌",
        color: Color::Red,
        description: "Failed",
    }
}
