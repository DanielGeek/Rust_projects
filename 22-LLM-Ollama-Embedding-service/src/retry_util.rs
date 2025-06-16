use std::time::Duration;

use anyhow::Error;
use log::{error, info};
use tokio::time::sleep;

pub async fn retry_with_backoff<F, Fut, T>(
    mut f: F,
    max_attempts: usize,
    initial_delay: Duration,
) -> Result<T, Error>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    let mut attempts = 0;
    let mut delay = initial_delay;

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_attempts => {
                info!("Attempt {} failed: {}.. Retrying..", attempts + 1, e);
                attempts += 1;
                sleep(delay).await;
                delay *= 2;
            }
            Err(e) => {
                error!("Attempt exhausted returning error");
                return Err(e);
            }
        }
    }
}
