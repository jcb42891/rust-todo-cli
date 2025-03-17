use crate::taskmanager::Task;
use std::process;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use std::path::Path;

const FILE_PATH: &str  = "tasks.txt";
const CURRENT_ID_FILE_PATH: &str = "id.txt";

pub fn write_task_to_file(description: &str) {
  let mut tasks_file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(FILE_PATH)
    .unwrap();

  let current_id = match get_current_id() {
    Ok(id) => id,
    Err(e) => {
      eprintln!("{e}");
      process::exit(1);
    } 
  };
  update_current_id(current_id+1);

  if let Err(e) = writeln!(tasks_file, "ID={};Description={}", current_id+1, description) {
    eprintln!("Couldn't write to the file: {}", e);
    process::exit(1);
  }
}

pub fn get_all_tasks_from_file() -> Vec<Task> {
 println!("Listing all tasks!");
 
 let contents = fs::read_to_string(FILE_PATH).unwrap_or_default();
 dbg!(&contents);
 let mut tasks: Vec<Task> = Vec::new();
 for line in contents.lines() {
  let parsed_task = parse_line(line);
  tasks.push(Task {id: parsed_task.0, description: parsed_task.1 });
 }
 tasks
}

pub fn remove_task_by_id_from_file(id: u64) -> bool {
  let contents = fs::read_to_string(FILE_PATH).unwrap_or_default();
  let mut lines_cache: Vec<&str> = Vec::new();
  let mut retval = false;

  for line in contents.lines() {
    let parsed_task: (u64, String) = parse_line(line);

    if parsed_task.0 == id {
      // This means we need to delete this line, so do not cache it
      retval = true;
    } else {
      // We need to cache this line since it's not getting deleted.
      lines_cache.push(line);
    }
  }
  // First remove the file
  let _ = fs::remove_file(FILE_PATH);

  let mut tasks_file = OpenOptions::new()
  .write(true)
  .append(true)
  .create(true)
  .open(FILE_PATH)
  .unwrap();

  for line in lines_cache {
    if let Err(e) = writeln!(tasks_file, "{line}") {
      eprintln!("Couldn't write to the file during task removal: {}", e);
      process::exit(1);
    }
  }

  return retval;
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
  let mut id_file = OpenOptions::new()
  .write(true)
  .append(false)
  .create(true)
  .open(CURRENT_ID_FILE_PATH)
  .unwrap();

  if let Err(e) = writeln!(id_file, "{}", id) {
    eprintln!("Couldn't write to the ID file: {}. Terminating.", e);
    process::exit(1);
  }
}

fn parse_line(line: &str) -> (u64, String) {
  let split_task: Vec<&str> = line.split(';').collect();
  let id_part = split_task[0];
  let desc_part = split_task[1];

  let id_str: &str = id_part.split('=').collect::<Vec<&str>>()[1];
  let desc_str: &str = desc_part.split('=').collect::<Vec<&str>>()[1];

  (id_str.parse::<u64>().expect("Could not parse ID from tasks file, file is likely corrupted!!"), desc_str.to_string())
}

