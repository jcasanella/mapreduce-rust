use proto::mapper::{GetNewTaskResponse, mapper_server::Mapper};
use tonic::{Request, Response, Status};

pub struct MapperService {}

impl MapperService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MapperService {
    fn default() -> Self {
        Self::new()
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
