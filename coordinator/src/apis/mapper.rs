use std::sync::Arc;

use proto::mapper::{GetNewTaskRequest, GetNewTaskResponse, mapper_server::Mapper};
use tonic::{Request, Response, Status};

use crate::coordinator_state::CoordinatorState;
use crate::mapper::coordinator_mapper::CoordinatorMapper;

#[allow(dead_code)]
pub struct MapperService {
    state: Arc<CoordinatorState>,
    coordinator_mapper: Arc<CoordinatorMapper>,
}

impl MapperService {
    pub fn new(state: Arc<CoordinatorState>, coordinator_mapper: Arc<CoordinatorMapper>) -> Self {
        Self {
            state,
            coordinator_mapper,
        }
    }
}

#[tonic::async_trait]
impl Mapper for MapperService {
    async fn get_new_task(
        &self,
        request: Request<GetNewTaskRequest>,
    ) -> Result<Response<GetNewTaskResponse>, Status> {
        let worker_id = request.into_inner().worker_id;

        // Validate that the worker is registered
        if !self.state.registered_workers.contains_key(&worker_id) {
            println!("Received heartbeat from unregistered worker: {}", worker_id);
            return Err(Status::not_found("Worker not registered"));
        }

        // Validate that there are remaining mappers to assign
        if self.coordinator_mapper.mappers_not_assigned.is_empty() {
            println!("No remaining mappers to assign to worker: {}", worker_id);
            return Ok(Response::new(GetNewTaskResponse {
                task_id: None,
                file_path: None,
            }));
        }

        // Look for an unassigned mapper and assign it to the worker
        let task_info = self.coordinator_mapper.mappers_not_assigned.pop();
        if let Some(task_info) = task_info {
            let task_name = task_info.task_name.clone();
            self.coordinator_mapper
                .add_mapper_to_map(&worker_id, task_info);
            println!("Assigned task {} to worker {}", task_name, worker_id);
            Ok(Response::new(GetNewTaskResponse {
                task_id: Some(worker_id),
                file_path: Some(task_name),
            }))
        } else {
            println!("No unassigned mappers available for worker: {}", worker_id);
            Err(Status::not_found("No unassigned mappers available"))
        }
    }
}
