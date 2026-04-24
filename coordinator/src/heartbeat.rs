use std::{time::Duration, sync::Arc};
use crate::coordinator_state::CoordinatorState;

pub async fn run(state: Arc<CoordinatorState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        println!("Checking heartbeats...");
        state.process_heartbeat();
    }
}