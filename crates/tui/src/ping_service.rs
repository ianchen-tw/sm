use log::{debug, info, warn};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant, sleep};

#[derive(Clone, Debug)]
pub enum PingResult {
    Success { latency: u32 },
    Failure { error: PingError },
}

#[derive(Clone, Debug)]
pub enum PingError {
    NoRouteToHost,
    CannotResolveName,
    NetworkUnreachable,
    RequestTimeout,
    HostUnreachable,
}

impl PingError {
    pub fn to_string(&self) -> &'static str {
        match self {
            PingError::NoRouteToHost => "No route to host",
            PingError::CannotResolveName => "Cannot resolve name",
            PingError::NetworkUnreachable => "Network unreachable",
            PingError::RequestTimeout => "Request timeout",
            PingError::HostUnreachable => "Host unreachable",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PingUpdate {
    pub server_name: String,
    pub result: PingResult,
    pub timestamp: Instant,
}

pub struct PingService {
    tx: mpsc::UnboundedSender<PingUpdate>,
    rx: mpsc::UnboundedReceiver<PingUpdate>,
}

impl PingService {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }

    pub fn get_receiver(&mut self) -> mpsc::UnboundedReceiver<PingUpdate> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.tx = tx;
        rx
    }

    pub fn start_ping_monitoring(&self, server_names: Vec<String>) -> tokio::task::JoinHandle<()> {
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut ping_tasks = Vec::new();

            // Start a ping task for each server
            for server_name in server_names {
                let tx_clone = tx.clone();
                let server_name_clone = server_name.clone();

                let task = tokio::spawn(async move {
                    Self::ping_server_loop(tx_clone, server_name_clone).await;
                });

                ping_tasks.push(task);
            }

            // Wait for all ping tasks to complete (they run indefinitely)
            for task in ping_tasks {
                let _ = task.await;
            }
        })
    }

    async fn ping_server_loop(tx: mpsc::UnboundedSender<PingUpdate>, server_name: String) {
        let mut rng = SmallRng::from_entropy();

        // Mix of reachable DNS servers and unreachable targets
        let dns_servers = [
            "8.8.8.8",         // Google DNS
            "8.8.4.4",         // Google DNS
            "1.1.1.1",         // Cloudflare DNS
            "1.0.0.1",         // Cloudflare DNS
            "9.9.9.9",         // Quad9 DNS
            "149.112.112.112", // Quad9 DNS
            "208.67.222.222",  // OpenDNS
            "208.67.220.220",  // OpenDNS
            "192.168.254.254", // Likely unreachable private IP
            "10.255.255.255",  // Likely unreachable private IP
            "172.31.255.255",  // Likely unreachable private IP
            "203.0.113.1",     // RFC 5737 - TEST-NET-3 (should be unreachable)
        ];

        loop {
            // Randomly select a DNS server to ping
            let target_server = dns_servers[rng.gen_range(0..dns_servers.len())];
            debug!("Pinging {target_server} for server {server_name}");

            // Perform actual ping
            let result = match IpAddr::from_str(target_server) {
                Ok(ip_addr) => match Self::ping_host(ip_addr).await {
                    Ok(latency) => {
                        debug!("Ping to {target_server} successful: {latency}ms");
                        PingResult::Success { latency }
                    }
                    Err(error) => {
                        warn!("Ping to {target_server} failed: {error:?}");
                        PingResult::Failure { error }
                    }
                },
                Err(_) => {
                    warn!("Cannot resolve address: {target_server}");
                    PingResult::Failure {
                        error: PingError::CannotResolveName,
                    }
                }
            };

            // Send the ping update
            let update = PingUpdate {
                server_name: server_name.clone(),
                result,
                timestamp: Instant::now(),
            };

            if tx.send(update).is_err() {
                // Channel closed, stop pinging
                break;
            }

            // Wait before next ping (vary interval slightly for realism)
            let ping_interval = Duration::from_millis(rng.gen_range(2000..=4000));
            sleep(ping_interval).await;
        }
    }

    async fn ping_host(ip_addr: IpAddr) -> Result<u32, PingError> {
        use tokio::process::Command;

        // Use system ping command for better reliability
        let output = Command::new("ping")
            .arg("-c")
            .arg("1") // Send only 1 packet
            .arg("-W")
            .arg("5") // 5 second timeout
            .arg(ip_addr.to_string())
            .output()
            .await;

        match output {
            Ok(output) => {
                if output.status.success() {
                    // Parse the ping output to extract latency
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    Self::parse_ping_output(&stdout)
                } else {
                    debug!("Ping to {ip_addr} failed: {output:?}");
                    // Check stderr for specific error types
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stderr.contains("Network is unreachable") {
                        Err(PingError::NetworkUnreachable)
                    } else if stderr.contains("Name or service not known") {
                        Err(PingError::CannotResolveName)
                    } else if stderr.contains("No route to host") {
                        Err(PingError::NoRouteToHost)
                    } else {
                        Err(PingError::RequestTimeout)
                    }
                }
            }
            Err(_) => Err(PingError::NetworkUnreachable),
        }
    }

    fn parse_ping_output(output: &str) -> Result<u32, PingError> {
        // Look for patterns like "time=123.456 ms" or "time=123ms"
        for line in output.lines() {
            if line.contains("time=") {
                if let Some(time_part) = line.split("time=").nth(1) {
                    if let Some(time_str) = time_part.split_whitespace().next() {
                        // Remove any trailing 'ms' and parse as float
                        let time_clean = time_str.trim_end_matches("ms");
                        if let Ok(time_f64) = time_clean.parse::<f64>() {
                            return Ok(time_f64 as u32);
                        }
                    }
                }
            }
        }

        // If we can't parse the output, assume it's a timeout
        Err(PingError::RequestTimeout)
    }

    pub async fn try_receive_update(&mut self) -> Option<PingUpdate> {
        self.rx.try_recv().ok()
    }
}

pub struct PingManager {
    server_results: HashMap<String, PingResult>,
    ping_receiver: mpsc::UnboundedReceiver<PingUpdate>,
}

impl PingManager {
    pub fn new(server_names: Vec<String>) -> (Self, tokio::task::JoinHandle<()>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut server_results = HashMap::new();

        // Initialize with default results
        let mut rng = SmallRng::from_entropy();
        for server_name in &server_names {
            let latency = rng.gen_range(10..=500);
            server_results.insert(server_name.clone(), PingResult::Success { latency });
        }

        // Start the ping monitoring task
        let ping_handle = tokio::spawn(async move {
            info!(
                "Starting ping monitoring for {} servers",
                server_names.len()
            );
            let mut ping_tasks = Vec::new();

            // Start a ping task for each server
            for server_name in server_names {
                let tx_clone = tx.clone();
                let server_name_clone = server_name.clone();

                let task = tokio::spawn(async move {
                    PingService::ping_server_loop(tx_clone, server_name_clone).await;
                });

                ping_tasks.push(task);
            }

            // Wait for all ping tasks to complete (they run indefinitely)
            for task in ping_tasks {
                let _ = task.await;
            }
        });

        let manager = Self {
            server_results,
            ping_receiver: rx,
        };

        (manager, ping_handle)
    }

    pub fn get_latency(&self, server_name: &str) -> Option<u32> {
        match self.server_results.get(server_name) {
            Some(PingResult::Success { latency }) => Some(*latency),
            Some(PingResult::Failure { .. }) => None,
            None => None,
        }
    }

    pub fn get_result(&self, server_name: &str) -> Option<&PingResult> {
        self.server_results.get(server_name)
    }

    pub fn is_server_reachable(&self, server_name: &str) -> bool {
        matches!(
            self.server_results.get(server_name),
            Some(PingResult::Success { .. })
        )
    }

    pub fn get_error_message(&self, server_name: &str) -> Option<&'static str> {
        match self.server_results.get(server_name) {
            Some(PingResult::Failure { error }) => Some(error.to_string()),
            _ => None,
        }
    }

    pub fn update_latencies(&mut self) {
        while let Ok(update) = self.ping_receiver.try_recv() {
            debug!("Received ping update: {update:?}");
            self.server_results
                .insert(update.server_name, update.result);
        }
    }
}
