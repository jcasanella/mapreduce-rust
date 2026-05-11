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
        Self { state, coordinator_mapper }
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
            println!(
                "Received heartbeat from unregistered worker: {}",
                worker_id
            );
            return Err(Status::not_found("Worker not registered"));
        }

        // Validate that there are remaining mappers to assign
        if self.coordinator_mapper.mappers_remaining <= 0 {
            println!("No remaining mappers to assign to worker: {}", worker_id);
            return Err(Status::not_found("No remaining mappers to assign"));
        }

        // Look for an unassigned mapper and assign it to the worker



        // TODO - implement logic to assign a new task to the worker based on the current state of mappers and workers  
        // with the worker_id we can check if the worker is registered and if it can be assigned a task, if so we can 
        // update the state of the mapper and return the task info to the worker
        //
        // Taks: if remaining mappers > 0 and worker is registered and can be assigned a task, then assign a task to the worker 
        // and update the state of the mapper and return the task info to the worker
        //
        // Iterate over the list of mappers and find the first one that is not assigned to a worker, if found assign it to the worker and update the state of the mapper, 
        // if not found return an error indicating that there are no tasks available


        println!("Received request for new task");
        Ok(Response::new(GetNewTaskResponse {
            task_id: 1,
            file_path: "input_data_for_task".to_string(),
        }))
    }
}
