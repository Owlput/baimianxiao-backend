use std::{sync::{atomic::AtomicU64, Arc}, time::Duration};

use tracing::info;

pub fn setup_code(code: Arc<AtomicU64>) {
    tokio::spawn(async move {
        loop {
            let val = rand::random::<u64>();
            code.store(val, std::sync::atomic::Ordering::SeqCst);
            info!("successfully generated access code:{}", val);
            tokio::time::sleep(Duration::from_secs(720)).await;
        }
    });
}
