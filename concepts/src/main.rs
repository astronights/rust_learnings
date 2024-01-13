use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to some basic Rust Concept Games!");
    println!("(1) Convert Temperature (C/F)");
    println!("(2) Generate the nth Fibonacci number");
    print!("Please pick an option: ");
    io::stdout().flush().expect("Could not flush stdout");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read your choice :(");

    match choice.trim().parse::<i32>().unwrap() {
        1 => convert_temp(),
        2 => fibonacci(),
        _ => println!("Invalid choice!"),
    }
}

fn convert_temp() {
    print!("Would you like to start from Celcius (C) or Fahrenheit (F): ");
    io::stdout().flush().expect("Could not flush stdout");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not read your choice :(");

    match choice.trim() {
        "C" => {
            print!("Enter a temperature in Celcius: ");
            io::stdout().flush().expect("Could not flush stdout");

            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Could not read your choice :(");
            let temp = temp.trim().parse::<f64>().unwrap();
            println!("{} C = {} F", temp, temp * 9.0 / 5.0 + 32.0);
            },
        "F" => {
            print!("Enter a temperature in Fahrenheit: ");
            io::stdout().flush().expect("Could not flush stdout");
            let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Could not read your choice :(");
            let temp = temp.trim().parse::<f64>().unwrap();
            println!("{} F = {} C", temp, (temp - 32.0) * 5.0 / 9.0);
            },
        _ => {
            println!("Invalid choice!");
            convert_temp();
        }
    }
}

fn fibonacci() {
    print!("Enter the nth Fibonacci number you would like to generate: ");
    io::stdout().flush().expect("Could not flush stdout");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Could not read your choice :(");
    let n = n.trim().parse::<i32>().unwrap();
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    println!("The {}th Fibonacci number is {}", n, c);
}