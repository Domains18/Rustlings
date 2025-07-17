use core::task;
use std::{
    os::unix::process,
    sync::atomic::{self, AtomicU64},
};

#[derive(Debug)]
pub struct Task {
    task: String,
    done_status: bool,
    id: u64,
}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    let command = args[0];

    match command {
        "add" => {
            if let Some(value) = args.get(1) {
                let new_task = *value;
                add_new_task(todo_list, new_task);
                display_todo(todo_list);
            } else {
                println!("please provide a new name for the task")
            }
        }

        "show" => {
            display_todo(todo_list);
        }

        "delete" => match &args[1].parse::<u64>() {
            Ok(value) => {
                remove_task(todo_list, *value);
            }

            Err(message) => {
                println!("{}", message.to_string());
            }
        },

        "update" => match &args[1].parse::<u64>() {
            Ok(value) => {
                if let Ok(task) = get_task(todo_list, *value) {
                    if let Some(value) = args.get(2) {
                        let new_task = *value;

                        task.update_task(new_task.into());
                    } else {
                        println!("no new task provided");
                    }
                } else {
                    println!("task not found in todo list");
                }
            }
            Err(message) => {
                println!("{}", message)
            }
        },

        "done" => match &args[1].parse::<u64>() {
            Ok(val) => {
                if let Ok(task) = get_task(todo_list, *val) {
                    task.update_status();
                } else {
                    println!("task id not found in list")
                }
            }
            Err(message) => {
                println!("{}", message.to_string());
            }
        },

        "exit" => {
            process::exit(0);
        }

        "help" => {
            display_help();
        }
    }
}

static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);
fn add_new_task(todo_list: &mut Vec<Task>, task_string: &str) {
    let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

    let task: Task = Task {
        task: task_string.into(),
        done_status: false,
        id: id_no,
    };

    todo_list.push(task);

    println!("{} added to the to do list: ", task_string)
}
