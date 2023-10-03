// Main features are: add, remove, done, sort, and reset.
use clap::Parser;
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(Parser)]
struct CliArgs {
    action: String,
    task_name: String,
}

fn main() {
    let args: CliArgs = CliArgs::parse();
    println!("Args: {}, {}", args.action, args.task_name);
    let mut todos: Vec<String> = Vec::new();

    read_database(&mut todos);
    println!("Read file: {:?}", todos);

    if args.action == "add" {
        add_todo(args.task_name, &mut todos);
    } else {
        println!("You chose another option\n Vector from DB: {:?}", todos);
    }
}

fn add_todo(todo: String, todo_vec: &mut Vec<String>) {
    let mut file_ref = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./list.td")
        .expect("Could not open file");
    let string_to_add = todo.clone();
    todo_vec.push(string_to_add);
    let string_write = todo + "\n";
    file_ref
        .write_all(string_write.as_bytes())
        .expect("Could not write to file");
    println!("Vector with added param: {:?}", todo_vec)
}

fn read_database(todo_vec: &mut Vec<String>) {
    let db_path = "./list.td";
    let todo_list_string = fs::read_to_string(db_path).expect("Could not open recent todos file");
    for line in todo_list_string.lines() {
        todo_vec.push(line.to_string());
    }
}
