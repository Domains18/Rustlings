use std::io;

fn main() {
    let mut choice;

    loop {
        println!("Temperature Converter.");
        println!("a) Celcius to Fahrenheit.");
        println!("b) Fahrenheit to Celcius.");
        println!("c) quit program.");
        println!("Enter your choice: ");

        //TODO: implement the functions
    }
}
 

 fun user_choice() -> string{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let num: f32 = user_input.trim().parse().expect("please enter a number")
 }
