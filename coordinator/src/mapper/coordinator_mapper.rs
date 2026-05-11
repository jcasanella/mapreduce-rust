use crossbeam::queue::SegQueue;
use dashmap::DashMap;

use crate::mapper::task_info::TaskInfo;
use std::{fs, io, path::Path};

#[allow(dead_code)]
pub struct CoordinatorMapper {
    mappers_map: DashMap<String, TaskInfo>,
    mappers_list: SegQueue<TaskInfo>,
    pub mappers_remaining: i32,
}

impl CoordinatorMapper {
    fn new() -> Self {
        CoordinatorMapper {
            mappers_map: DashMap::new(),
            mappers_list: SegQueue::new(),
            mappers_remaining: 0,
        }
    }

    fn add_mapper(&mut self, task_name: &str) {
        self.mappers_list.push(TaskInfo::new(task_name));
        self.mappers_remaining += 1;
    }


    // #[allow(dead_code)]
    // fn complete_mapper(&mut self, worker_id: &String) {
    //     if let Some(mapper_info) = self.mappers.get_mut(worker_id) {
    //         mapper_info.complete();
    //         self.mappers_remaining -= 1;
    //     }
    // }

    // #[allow(dead_code)]
    // fn fail_mapper(&mut self, worker_id: &String, error_message: String) {
    //     if let Some(mapper_info) = self.mappers.get_mut(worker_id) {
    //         mapper_info.fail(error_message);
    //         self.mappers_remaining -= 1;
    //     }
    // }
}

impl Default for CoordinatorMapper {
    fn default() -> Self {
        Self::new()
    }
}

pub fn setup_mappers(dir: &Path) -> io::Result<CoordinatorMapper> {
    let mut coordinator_mapper = CoordinatorMapper::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && let Some(file_name) = path.file_name().and_then(|n| n.to_str())
            {
                println!("Adding mapper for file: {}", file_name);
                coordinator_mapper.add_mapper(file_name);
            }
        }

        println!(
            "Finished setting up mappers. Total mappers: {}",
            coordinator_mapper.mappers_remaining
        );

        Ok(coordinator_mapper)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir.display()),
        ))
    }
}
