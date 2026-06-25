use proto::mapper;
use proto::mapper::{GetNewTaskRequest, GetNewTaskResponse, mapper_client::MapperClient};
use std::{fs::File, io::BufReader, io::BufRead};
use crate::map::word_count::WordCountMapper;

use crate::config;

pub async fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error + Send>> {
    let mut has_task = false;
    let mut mapper_client = MapperClient::connect(config.coordinator_addr.clone())
        .await
        .expect("Failed to connect mapper client");
    
    let mut file_name: Option<String> = None;

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
            match (task_id, file_path) {
                (Some(task_id), Some(file_path)) => {
                    println!("Received new task: id={}, file_path={}", task_id, file_path);
                    file_name = Some(file_path);
                }
                _ => {
                    println!("No new task assigned, will check again later.");
                    has_task = false;
                }
            }
        } else if let Some(name) = &file_name {
            println!("Already has a task assigned, skipping request for new task.");
            let mapper = WordCountMapper;
            match File::open(name) {
                Ok(file) => {
                    let reader = BufReader::new(file);

                    for line in reader.lines() {
                        match line {
                            Ok(l) => mapper.map(l),
                            Err(e) => eprintln!("Error reading line: {}", e),
                        }
                    }
                }
                Err(e) => eprint!("Error opening file {}: {}", name, e),
            }
            
        }
    }
}
