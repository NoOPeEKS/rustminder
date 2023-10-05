// Main features are: add, remove, show.
use clap::Parser;

fn main() {
    run();
}

#[derive(Parser)]
struct CliArgs {
    action: String,
    task_name: String,
}

fn run() {
    let args: CliArgs = CliArgs::parse();
    println!("Args: {}, {}", args.action, args.task_name);
    let mut todos: Vec<String> = Vec::new();

    rustminder::read_database(&mut todos);
    println!("Read file: {:?}", todos);

    if args.action == "add" {
        rustminder::add_todo(args.task_name, &mut todos);
    } else if args.action == "remove" {
        if args.task_name != "all" {
            rustminder::remove_todo(args.task_name, &mut todos);
        } else {
            rustminder::remove_all();
        }
    } else if args.action == "show" && args.task_name == "all" {
        rustminder::show_todos(&todos);
    } else {
        println!("Wrong parameters or functionality not yet implemented!");
    }
}
