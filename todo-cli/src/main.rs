extern crate todo_cli;
use todo_cli::Task;


use std::io::stdin;
use std::io::stdout;
use std::io::Write;



fn runprompt(todo: &mut Vec<Task>){
    loop {
        let mut stdout = stdout();
        print!("(todo list) > ");
        stdout.flush().expect("can't flush the stdout");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("cannot readline");

        let args: Vec<&str> = buffer.split_whitespace().collect();

        todo_cli::run(args, todo);
    }
}

fn main(){
    let mut todo: Vec<Task> = Vec::new();

    runprompt(&mut todo);
}