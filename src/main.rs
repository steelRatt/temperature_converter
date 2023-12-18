use std::io;

fn get_input(prompt: &str) -> f64 {
    loop {
        // Display prompt, get input
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // Check if can be parsed to f64 using match
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        }
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn main() {
    println!("Welcome to Temperature Converter!");

    loop {
        println!("---------------------------------");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let choice = get_input("Enter (1 or 2):");

        let result = match choice as i32 {
            1 => Some(celsius_to_fahrenheit(get_input("Enter temperature in C:"))),
            2 => Some(fahrenheit_to_celsius(get_input("Enter temperature in F:"))),
            _ => {
                println!("Invalid choice.");
                continue;
            }
        };

        match result {
            Some(value) => println!("Converted temp: {}", value),
            None => println!("Error Converting."),
        }

        println!("Perform another conversion (yes/no):");
        let mut go_again = String::new();
        io::stdin()
            .read_line(&mut go_again)
            .expect("Failed to read line.");

        if go_again.trim().eq_ignore_ascii_case("no") {
            break;
        }
    }
}
