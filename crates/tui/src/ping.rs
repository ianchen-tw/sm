use ratatui::style::Color;

#[derive(Copy, Clone)]
pub struct PingInfo {
    pub emoji: &'static str,
    pub color: Color,
    pub description: &'static str,
}

impl PingInfo {
    pub fn format_with_latency(&self, latency: u32) -> String {
        format!("{}ms", latency)
    }
}

pub struct PingThreshold {
    pub max_latency: u32,
    pub info: PingInfo,
}

/// Define ping latency thresholds and their visual representation.
/// 
/// To modify latency ranges, colors, or emojis, update this function.
/// The thresholds are checked in order - the first matching threshold is used.
/// Keep them sorted from lowest to highest latency.
pub fn get_ping_thresholds() -> Vec<PingThreshold> {
    vec![
        PingThreshold {
            max_latency: 50,
            info: PingInfo {
                emoji: "⚡",
                color: Color::Cyan,
                description: "Excellent",
            },
        },
        PingThreshold {
            max_latency: 100,
            info: PingInfo {
                emoji: "🟢",
                color: Color::Green,
                description: "Good",
            },
        },
        PingThreshold {
            max_latency: 200,
            info: PingInfo {
                emoji: "🟡",
                color: Color::Yellow,
                description: "OK",
            },
        },
        PingThreshold {
            max_latency: 350,
            info: PingInfo {
                emoji: "🟠",
                color: Color::LightRed,
                description: "Slow",
            },
        },
    ]
}

/// Define the visual representation for poor ping (above all thresholds).
/// 
/// To modify the appearance of servers with high latency, update this function.
pub fn get_ping_poor() -> PingInfo {
    PingInfo {
        emoji: "🔴",
        color: Color::Red,
        description: "Poor",
    }
}

/// Generate a legend string showing all ping categories and their thresholds.
pub fn get_ping_legend() -> String {
    let thresholds = get_ping_thresholds();
    let ping_poor = get_ping_poor();
    
    let mut legend = String::from("Ping: ");
    let mut first = true;
    
    for threshold in &thresholds {
        if !first {
            legend.push_str(" | ");
        }
        legend.push_str(&format!(
            "{} {} (<{}ms)",
            threshold.info.emoji,
            threshold.info.description,
            threshold.max_latency
        ));
        first = false;
    }
    
    legend.push_str(&format!(
        " | {} {} (>{}ms)",
        ping_poor.emoji,
        ping_poor.description,
        thresholds.last().unwrap().max_latency
    ));
    
    legend
}

/// Get the appropriate ping info for a given latency value.
pub fn get_ping_info_for_latency(latency: u32) -> PingInfo {
    let thresholds = get_ping_thresholds();
    for threshold in thresholds {
        if latency <= threshold.max_latency {
            return threshold.info;
        }
    }
    get_ping_poor()
} 