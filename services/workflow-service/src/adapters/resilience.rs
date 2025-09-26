use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct CircuitConfig {
    pub fail_threshold: u32,
    pub open_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open(Instant),
    HalfOpen,
}

#[derive(Clone)]
pub struct CircuitBreaker {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
struct Inner {
    services: HashMap<String, ServiceCircuit>,
    cfg: CircuitConfig,
}

#[derive(Debug, Clone)]
struct ServiceCircuit {
    state: CircuitState,
    consecutive_failures: u32,
}

impl Default for ServiceCircuit {
    fn default() -> Self {
        Self { state: CircuitState::Closed, consecutive_failures: 0 }
    }
}

impl CircuitBreaker {
    pub fn new(cfg: CircuitConfig) -> Self {
        Self { inner: Arc::new(Mutex::new(Inner { services: HashMap::new(), cfg })) }
    }

    pub fn can_call(&self, service: &str) -> bool {
        let mut inner = self.inner.lock().unwrap();
        let open_seconds = inner.cfg.open_seconds;
        let entry = inner.services.entry(service.to_string()).or_default();
        match entry.state {
            CircuitState::Closed => true,
            CircuitState::HalfOpen => true, // allow a probe
            CircuitState::Open(since) => {
                if since.elapsed() >= Duration::from_secs(open_seconds) {
                    entry.state = CircuitState::HalfOpen;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn record_success(&self, service: &str) {
        let mut inner = self.inner.lock().unwrap();
        let entry = inner.services.entry(service.to_string()).or_default();
        entry.consecutive_failures = 0;
        entry.state = CircuitState::Closed;
    }

    pub fn record_failure(&self, service: &str) {
        let mut inner = self.inner.lock().unwrap();
        let fail_threshold = inner.cfg.fail_threshold;
        let entry = inner.services.entry(service.to_string()).or_default();
        entry.consecutive_failures += 1;
        if entry.consecutive_failures >= fail_threshold {
            entry.state = CircuitState::Open(Instant::now());
        }
    }
}

// Global singleton (simple)
use once_cell::sync::Lazy;
static CB: Lazy<CircuitBreaker> = Lazy::new(|| CircuitBreaker::new(CircuitConfig { fail_threshold: 5, open_seconds: 30 }));

pub fn can_call(service: &str) -> bool { CB.can_call(service) }
pub fn record_success(service: &str) { CB.record_success(service) }
pub fn record_failure(service: &str) { CB.record_failure(service) }
