use clap::Parser;
use todo_cli::cmdparser::Args;
use todo_cli::cmdparser::Commands;
use todo_cli::taskmanager;

fn main() {
  let args = Args::parse();

  match args.command {
    Some(Commands::Add { description }) => {
      let _ = taskmanager::add_task(&description);
    },
    Some(Commands::List {  }) => {
      let _ = taskmanager::list_tasks();
    },
    Some(Commands::Remove { task_id }) => {
      let _ = taskmanager::remove_task(task_id);
    },
    None => {
      Args::parse_from(&["todo_cli", "--help"]);
      std::process::exit(1);
    }
  }
}
