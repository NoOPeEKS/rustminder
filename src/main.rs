// Main features are: add, remove, done, sort, and reset.
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
struct CliArgs {
    action: String,
    task_name: String,
}

fn main() {
    let args: CliArgs = CliArgs::parse();
    println!("Args: {}, {}", args.action, args.task_name);
    let mut todos: Vec<String> = Vec::new();

    rustminder::read_database(&mut todos);
    println!("Read file: {:?}", todos);

    if args.action == "add" {
        rustminder::add_todo(args.task_name, &mut todos);
    } else if args.action == "show" {
        rustminder::show_todos(&mut todos);
    } else {
        println!("Invalid argument or not implemented yet!");
    }
}
