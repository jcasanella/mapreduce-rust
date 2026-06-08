use crate::coordinator_state::CoordinatorState;
use std::{sync::Arc, time::Duration};

pub async fn run(state: Arc<CoordinatorState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        println!("Checking heartbeats...");
        state.process_heartbeat();
    }
}
