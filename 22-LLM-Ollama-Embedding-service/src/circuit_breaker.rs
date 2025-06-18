use anyhow::anyhow;
use anyhow::Error;
use log::error;
use log::info;
use log::warn;
use std::time::{Duration, Instant};

enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

pub struct CircuitBreaker {
    state: CircuitBreakerState,
    failure_count: usize,
    success_count: usize,
    failure_threshold: usize,
    success_threshold: usize,
    cooldown_period: Duration,
    last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(
        failure_threshold: usize,
        success_threshold: usize,
        cooldown_period: Duration,
    ) -> Self {
        CircuitBreaker {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            failure_threshold,
            success_threshold,
            cooldown_period,
            last_failure_time: None,
        }
    }

    pub async fn call<F, Fut, T>(&mut self, mut operation: F) -> Result<T, Error>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        match self.state {
            CircuitBreakerState::Open => {
                if let Some(last_failure_time) = self.last_failure_time {
                    if last_failure_time.elapsed() >= self.cooldown_period {
                        self.state = CircuitBreakerState::HalfOpen;
                        self.success_count = 0;
                        warn!("Circuit Breaker transition to half open state");
                    } else {
                        warn!("Circuit Breaker is open.. Requests are blocked for now");
                        return Err(anyhow!(
                            "Circuit Breaker is open. Please try later..!".to_string()
                        ));
                    }
                }
            }
            _ => {}
        }

        match operation().await {
            Ok(result) => {
                info!("Request Sucess response");
                self.on_success();
                Ok(result)
            }
            Err(err) => {
                error!("Failed with {}", err);
                self.on_failure();
                Err(err)
            }
        }
    }

    fn on_success(&mut self) {
        match self.state {
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitBreakerState::Closed;
                    self.failure_count = 0;
                    info!("Circuit Breaker transitioning to closed state");
                }
            }
            _ => {
                self.success_count = 0;
            }
        }
    }

    fn on_failure(&mut self) {
        self.failure_count += 1;
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitBreakerState::Open;
            self.last_failure_time = Some(Instant::now());
            error!("Circuit Breaker transitioning to open state");
        }
    }
}
