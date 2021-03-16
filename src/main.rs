pub mod akso;
mod habitica_todo;

use clap::{App, Arg};
use crate::habitica_todo::HabiticaTodos;
use crate::akso::{Todos, Todo};

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
                .value_name("User todos")
                .takes_value(false)
                .help("Show the current todos of this user")
        )
        .arg(
            Arg::with_name("finish")
                .short("f")
                .long("finish")
                .value_name("Finish a todo")
                .takes_value(true)
                .help("Mark a todo as completed.")
        )
        .arg(
            Arg::with_name("create")
                .short("create")
                .long("create")
                .value_name("Create a todo")
                .takes_value(true)
                .help("Create a todo.")
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
}

fn fetch_todo(todos: &dyn Todos) {
    println!("tasks: {}", todos.all().len());
    let mut index = 0;
    todos.all().iter().for_each(|todo| {
        index = index + 1;
        println!("{} {}", index, todo.title())
    });
}

fn finish_task(todos: &dyn Todos, id: String) {
    let idx:usize = id.parse().unwrap();
    let all = todos.all();
    let selected = all.get(idx).unwrap();
    println!("Finish todo: {}", selected.title());
    todos.finish(selected.id());
}

fn create_todo(todos: &dyn Todos, title: String){
    todos.create(title);
    fetch_todo(todos);
}