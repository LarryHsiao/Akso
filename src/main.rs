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
            Arg::with_name("tasks")
                .short("t")
                .long("tasks")
                .value_name("User tasks")
                .takes_value(false)
                .help("Show the current tasks of this user")
        )
        .get_matches();
    let todos = HabiticaTodos{
        api_key: matches.value_of("api_key").unwrap().to_string(),
        user_id: matches.value_of("user_id").unwrap().to_string()
    };
    let task_cmd = matches.index_of("tasks");
    if task_cmd.is_some() {
        listing(&todos)
    }
}

fn listing(todos: &dyn Todos) {
    println!("tasks: {}", todos.all().len());
    todos.all().iter().for_each(|todo| {
        println!("{} {}", &todo.id()[..5], todo.title())
    });
}
