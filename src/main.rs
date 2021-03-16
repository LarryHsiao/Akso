pub mod akso;
mod habitica_todo;

use clap::{App, Arg};
use crate::habitica_todo::HabiticaTodos;
use crate::akso::Todos;

fn main() {
    let matches = App::new("akso")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("api_key")
                .short("api_key")
                .long("api_key")
                .value_name("Api key")
                .required(true)
                .takes_value(true)
                .help("Api key for accessing the Habitica data.")
        )
        .arg(
            Arg::with_name("user_id")
                .short("user_id")
                .long("user_id")
                .value_name("User ID")
                .required(true)
                .takes_value(true)
                .help("User Id for accessing the Habitica data.")
        )
        .arg(
            Arg::with_name("todos")
                .short("t")
                .long("todos")
                .takes_value(false)
                .help("Show the current todos of current user")
        )
        .arg(
            Arg::with_name("finish")
                .short("f")
                .long("finish")
                .value_name("Index of Todo")
                .takes_value(true)
                .help("Mark a todo as completed.")
        )
        .arg(
            Arg::with_name("create")
                .short("create")
                .long("create")
                .value_name("Title")
                .takes_value(true)
                .help("Create a todo.")
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("Index of the todo")
                .takes_value(true)
                .help("Delete a todo by given id")
        )
        .get_matches();
    let todos = HabiticaTodos {
        api_key: matches.value_of("api_key").unwrap().to_string(),
        user_id: matches.value_of("user_id").unwrap().to_string(),
    };
    let task_cmd = matches.index_of("todos");
    if task_cmd.is_some() {
        fetch_todo(&todos)
    }
    let finish_cmd = matches.index_of("finish");
    if finish_cmd.is_some() {
        finish_task(
            &todos,
            matches.value_of("finish").unwrap().to_string(),
        )
    }
    let create_cmd = matches.index_of("create");
    if create_cmd.is_some() {
        create_todo(
            &todos,
            matches.value_of("create").unwrap().to_string(),
        )
    }
    let delete_cmd = matches.index_of("delete");
    if delete_cmd.is_some() {
        delete_todo(
            &todos,
            matches.value_of("delete").unwrap().to_string(),
        )
    }
}

fn fetch_todo(todos: &dyn Todos) {
    println!("tasks: {}", todos.all().len());
    let mut index = 0;
    todos.all().iter().for_each(|todo| {
        index = index + 1;
        println!("{} {}", index, todo.title())
    });
}

fn finish_task(todos: &dyn Todos, index: String) {
    let idx: usize = (index.parse::<i32>().unwrap() - 1) as usize;
    let all = todos.all();
    let selected = all.get(idx).unwrap();
    println!("Todo finished: {}", selected.title());
    todos.finish(selected.id());
    fetch_todo(todos);
}

fn create_todo(todos: &dyn Todos, title: String){
    todos.create(title);
    fetch_todo(todos);
}

fn delete_todo(todos: &dyn Todos, index: String) {
    let idx: usize = (index.parse::<i32>().unwrap() - 1) as usize;
    let all = todos.all();
    let selected = all.get(idx).unwrap();
    println!("Todo deleted: {}", selected.title());
    todos.delete(selected.id());
    fetch_todo(todos);
}