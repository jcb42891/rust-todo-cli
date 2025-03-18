use crate::taskmanager::Task;
use std::process;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use std::fs;
use std::path::Path;

const TASKS_FILE_PATH: &str  = "tasks.txt";
const CURRENT_ID_FILE_PATH: &str = "id.txt";
const TEMP_FILE: &str = "temp.txt";

pub fn write_task_to_file(description: &str) -> Result<(), io::Error> {
  let mut tasks_file: fs::File = get_file_handle_for_write(TASKS_FILE_PATH, true);

  let current_id: u64 = match get_current_id() {
    Ok(id) => id,
    Err(e) => {
      eprintln!("{e}");
      process::exit(1);
    } 
  };
  update_current_id(current_id+1);

  writeln!(tasks_file, "ID={};Description={}", current_id+1, description).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
  Ok(())
}

pub fn get_all_tasks_from_file() -> Result<Vec<Task>, io::Error> {
 println!("Listing all tasks!");

 let file = fs::File::open(TASKS_FILE_PATH);
 let reader = BufReader::new(file?);
 let mut tasks = Vec::new();

 for line in reader.lines() {
  let line = line?;
  let task = parse_line(&line).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
  tasks.push(task);
 }
 Ok(tasks)
}

pub fn remove_task_by_id_from_file(id: u64) -> Result<bool, io::Error> {
  let file = fs::File::open(TASKS_FILE_PATH);
  let reader = BufReader::new(file?);
  let mut lines_cache: Vec<String> = Vec::new();
  let mut retval = false;

  for line in reader.lines() {
    let line = line?;
    let line_clone = line.clone();
    let parsed_task: Task = parse_line(&line_clone).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if parsed_task.id == id {
      // This means we need to delete this line, so do not cache it
      retval = true;
    } else {
      // We need to cache this line since it's not getting deleted.
      lines_cache.push(line);
    }
  }
  // First copy the existing task file to a tmp file
  fs::copy(TASKS_FILE_PATH, TEMP_FILE)?;
  // Next remove the tasks file
  let _ = fs::remove_file(TASKS_FILE_PATH);
  // Rebuild the tasks file
  let mut tasks_file: fs::File = get_file_handle_for_write(TASKS_FILE_PATH, true);
  for line in lines_cache {
    writeln!(tasks_file, "{line}").map_err(|e| {
      // If there's an error, we should roll back to the tmp file.
      let _ = fs::copy(TEMP_FILE, TASKS_FILE_PATH);
      io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "There was a problem adding a new task, reverted back to original tasks list. Error: {}",
            e
        )
    )
    })?;
  }

  Ok(retval)
}

fn get_current_id() -> Result<u64, String>  {
  let is_new_file= !Path::new(CURRENT_ID_FILE_PATH).exists();
  if is_new_file {
    return Ok(0);
  } else {
    // Read the first line of the file
    let contents = fs::read_to_string(CURRENT_ID_FILE_PATH).unwrap_or_default();
    let first_line = contents.lines().next().unwrap_or("");
    println!("Contents {}", contents);
    println!("first line {}", first_line);
   
   first_line.parse::<u64>().map_err(|_| format!("Corrupted data file! Could not parse current task ID"))
  }
}

fn update_current_id(id: u64) {
  let mut id_file = get_file_handle_for_write(CURRENT_ID_FILE_PATH, false);

  if let Err(e) = writeln!(id_file, "{}", id) {
    eprintln!("Couldn't write to the ID file: {}. Terminating.", e);
    process::exit(1);
  }
}

fn parse_line(line: &str) -> Result<Task, String> {
  let split_task: Vec<&str> = line.split(';').collect();
  if split_task.len() != 2 {
    return Err("Invalid task format".to_string());
  }
  let id_part = split_task[0].split('=').nth(1).ok_or("Missing ID")?;
  let desc_part = split_task[1].split('=').nth(1).ok_or("Missing description")?;
  let id = id_part.parse::<u64>().map_err(|_| "Invalid ID")?;

  Ok(
    Task { id: id, description: desc_part.to_string()}
  )
}

fn get_file_handle_for_write(file_path: &str, should_append: bool) -> std::fs::File {
  OpenOptions::new()
    .write(true)
    .append(should_append)
    .create(true)
    .open(file_path)
    .unwrap()
}
