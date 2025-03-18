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
    #[arg(required = true)]
    description: String,
  },
  /// Lists all existing tasks
  List {},
  /// Removes a task by its task id
  Remove {
    #[arg(required = true, num_args = 1, value_parser=clap::value_parser!(u64))]
    task_id: u64,
  }
}