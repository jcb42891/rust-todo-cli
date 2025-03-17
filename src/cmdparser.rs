use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about="A simple todo list app for the CLI", long_about = None)]
pub struct Args {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Adds a new task with a description
  Add {
    #[arg(required = true, num_args = 1, value_parser=parse_description)]
    description: String,
  },
  /// Lists all existing tasks
  List {},
  /// Removes a task by its task id
  Remove {
    #[arg(required = true, num_args = 1, value_parser=parse_task_id)]
    task_id: u64,
  }
}

fn parse_description(s: &str) -> Result<String, String>{
  if s.trim().len() == 0 {
    return Err(format!("Please enter a non-empty string."));
  }
  s.trim().parse::<String>().map_err(|_| format!("Please enter a valid string."))
}

fn parse_task_id(id: &str) -> Result<u64, String> {
  id.trim().parse::<u64>().map_err(|_| format!("{} is not a valid task ID format, please enter an integer value", id))
}