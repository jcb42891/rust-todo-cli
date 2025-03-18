use crate::filemanager;
use std::io;

pub struct Task {
  pub id: u64,
  pub description: String
}

pub fn add_task(description: &str) -> Result<(), io::Error> {
  filemanager::write_task_to_file(description)?;
  Ok(())
}

pub fn list_tasks() -> Result<(), Box<dyn std::error::Error>> {
 for task in  filemanager::get_all_tasks_from_file()? {
  println!("ID: {}, Description: {}", task.id, task.description);
 }
 Ok(())
}

pub fn remove_task(id: u64) -> Result<(), Box<dyn std::error::Error>> {
  let result = filemanager::remove_task_by_id_from_file(id)?;
  if !result {
    println!("No task found with id {id}, so nothing was deleted")
  } else {
    println!("Removing task id {}", id);
  }
  Ok(())
}