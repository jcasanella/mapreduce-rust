use proto::mapper::{GetNewTaskRequest, GetNewTaskResponse, mapper_client::MapperClient};

use crate::config;

pub async fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error + Send>> {
    let mut has_task = false;
    let mut mapper_client = MapperClient::connect(config.coordinator_addr.clone())
        .await
        .expect("Failed to connect mapper client");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        if !has_task {
            println!("Requesting new task...");
            let response = mapper_client
                .get_new_task(tonic::Request::new(GetNewTaskRequest {
                    worker_id: config.worker_id.clone(),
                }))
                .await
                .expect("Failed to get new task");

            has_task = true;

            let GetNewTaskResponse { task_id, file_path } = response.into_inner();
            println!("Received new task: id={}, file_path={}", task_id, file_path);
        } else {
            println!("Already has a task assigned, skipping request for new task.");
        }
    }
}
