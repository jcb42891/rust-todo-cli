use crate::filemanager;

pub struct Task {
  pub id: u64,
  pub description: String
}

pub fn add_task(description: &str) {
  filemanager::write_task_to_file(description);
}

pub fn list_tasks() {
 for task in  filemanager::get_all_tasks_from_file() {
  println!("ID: {}, Description: {}", task.id, task.description);
 }
}

pub fn remove_task(id: u64) -> bool {
  let result = filemanager::remove_task_by_id_from_file(id);
  if !result {
    println!("No task found with id {id}, so nothing was deleted")
  } else {
    println!("Removing task id {}", id);
  }
  result
}