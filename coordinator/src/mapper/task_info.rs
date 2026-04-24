enum Status {
    Complete,
    InProgress,
    Failed(String), // Include an error message for failed status
    NotStarted,
}



pub struct TaskInfo {
    task_id: String,
    status: Status,
    start_time: Option<prost_types::Timestamp>,
    end_time: Option<prost_types::Timestamp>,
}

impl TaskInfo {
    pub fn new(task_id: &str) -> Self {
        TaskInfo {
            task_id: task_id.to_string(),
            status: Status::NotStarted,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.status = Status::InProgress;
        self.start_time = Some(prost_types::Timestamp::from(std::time::SystemTime::now())); 
    }

    pub fn complete(&mut self) {
        self.status = Status::Complete;
        self.end_time = Some(prost_types::Timestamp::from(std::time::SystemTime::now())); 
    }

    pub fn fail(&mut self, error_message: String) {
        self.status = Status::Failed(error_message);
        self.end_time = Some(prost_types::Timestamp::from(std::time::SystemTime::now())); 
    }
}