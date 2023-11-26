use std::io;

fn main() {
    println!("Give me a number for generate the nth Fibonacci number");

    let mut number = String::new(); 

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line !");
    
    let number: i32 = number.trim().parse().expect("Failed to read");

    let result: i32 = fibonacci(number);

    println!("The nth Fibonacci number is: {result}");
}

fn fibonacci(num: i32) -> i32 {
    match num {
        0 => 1,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}