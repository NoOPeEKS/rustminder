use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn read_database(todo_vec: &mut Vec<String>) {
    let db_path = "./list.td";
    let todo_list_string = fs::read_to_string(db_path).expect("Could not open recent todos file");
    for line in todo_list_string.lines() {
        todo_vec.push(line.to_string());
    }
}

pub fn add_todo(todo: String, todo_vec: &mut Vec<String>) {
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

pub fn show_todos(todo_vec: &mut Vec<String>) {
    for item in todo_vec {
        println!("{item}");
    }
}
