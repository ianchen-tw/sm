use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, Instant};
use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::collections::HashMap;

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
        
        loop {
            // 10% chance of failure, 90% success
            let result = if rng.gen_bool(0.1) {
                // Generate random failure
                let errors = [
                    PingError::NoRouteToHost,
                    PingError::CannotResolveName,
                    PingError::NetworkUnreachable,
                    PingError::RequestTimeout,
                    PingError::HostUnreachable,
                ];
                let error = errors[rng.gen_range(0..errors.len())].clone();
                PingResult::Failure { error }
            } else {
                // Generate successful ping with random latency between 10-300ms
                let latency = rng.gen_range(10..=300);
                PingResult::Success { latency }
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
            
            // Wait before next ping (simulate ping interval)
            let ping_interval = Duration::from_millis(rng.gen_range(1000..=3000));
            sleep(ping_interval).await;
        }
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
        matches!(self.server_results.get(server_name), Some(PingResult::Success { .. }))
    }

    pub fn get_error_message(&self, server_name: &str) -> Option<&'static str> {
        match self.server_results.get(server_name) {
            Some(PingResult::Failure { error }) => Some(error.to_string()),
            _ => None,
        }
    }

    pub fn update_latencies(&mut self) {
        while let Ok(update) = self.ping_receiver.try_recv() {
            self.server_results.insert(update.server_name, update.result);
        }
    }
} 