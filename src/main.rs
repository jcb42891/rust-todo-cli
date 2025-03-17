use clap::Parser;
use todo_cli::cmdparser::Args;
use todo_cli::cmdparser::Commands;
use todo_cli::taskmanager;

fn main() {
  let args = Args::parse();

  match args.command {
    Some(Commands::Add { description }) => {
      taskmanager::add_task(&description);
    },
    Some(Commands::List {  }) => {
      taskmanager::list_tasks();
    },
    Some(Commands::Remove { task_id }) => {
      taskmanager::remove_task(task_id);
    },
    None => println!("Provided command not supported")
  }
}
