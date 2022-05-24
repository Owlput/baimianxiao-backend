use std::time::Duration;

use tokio::sync::broadcast;
use tracing::{error, info};

pub fn setup_code_store() -> broadcast::Receiver<u64> {
    let (tx, rx) = broadcast::channel(16);
    tokio::spawn(async move {
        loop {
            let generated_code = rand::random::<u64>();
            match tx.send(generated_code) {
                Ok(_) => {
                    info!("A new access code has been generated:{}", generated_code);
                }
                Err(e) => {
                    error!("Failed to generate a new access code:{}", e);
                }
            }
            tokio::time::sleep(Duration::from_secs(3600)).await;
        }
    });
    rx
}
