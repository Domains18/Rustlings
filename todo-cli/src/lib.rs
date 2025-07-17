use std::{
    process,
    sync::atomic::{self, AtomicU64},
};

#[derive(Debug)]
pub struct Task {
    task: String,
    done_status: bool,
    id: u64,
}

impl Task {
    fn update_status(&mut self) {
        self.done_status = true
    }

    fn update_task(&mut self, new_name: String) {
        self.task = new_name;
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

fn display_todo(todo_list: &mut Vec<Task>) {
    if todo_list.len() < 1 {
        println!("empty todo list");
        return;
    }

    for i in todo_list {
        println!("id: {}, name: {}, done: {}", i.id, i.task, i.done_status);
    }
}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64) {
    todo_list.retain(|task| task.id != id_no);
}

fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str> {
    for t in todo_list {
        if t.id == task_id {
            return Ok(t);
        } else {
            continue;
        }
    }
    return Err("Task not found in the todolist");
}

fn display_help() {
    let help: &str = "
        to add help here
    ";

    println!("{}", help)
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

        _ => {
            println!("Unknown command. Use 'help' to see available commands.");
            display_help();
        }
    }
}


pub fn run(args: Vec<&str>, todo: &mut Vec<Task>){
    parse_arguments(args, todo);
}