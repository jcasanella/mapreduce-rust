use std::sync::Arc;

use proto::mapper::{GetNewTaskResponse, mapper_server::Mapper};
use tonic::{Request, Response, Status};

use crate::coordinator_state::CoordinatorState;

#[allow(dead_code)]
pub struct MapperService {
    state: Arc<CoordinatorState>,
}

impl MapperService {
    pub fn new(state: Arc<CoordinatorState>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Mapper for MapperService {
    async fn get_new_task(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetNewTaskResponse>, Status> {
        println!("Received request for new task");
        Ok(Response::new(GetNewTaskResponse {
            task_id: 1,
            file_path: "input_data_for_task".to_string(),
        }))
    }
}
