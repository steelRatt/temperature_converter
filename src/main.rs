use std::io;

// Function to get input for temperatures
fn get_input(prompt: &str) -> f64 {
    loop {
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

// Functions to convert temps
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

// Function to get conversion choice from user
fn get_choice() -> i32 {
    loop {
        println!("Enter (1 or 2):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<i32>() {
            Ok(num) if 1 <= num && num <= 2 => return num,
            _ => println!("Invalid choice."),
        }
    }
}

// Function to handle conversion logic based on choice
fn perform_conversion(choice: i32) {
    if choice == 1 {
        let temp = get_input("\nEnter temperature in C:");
        let converted = celsius_to_fahrenheit(temp);
        println!("\n{} Celsius is {} Fahrenheit.", temp, converted);
    } else if choice == 2 {
        let temp = get_input("\nEnter temperature in F:");
        let converted = fahrenheit_to_celsius(temp);
        println!("\n{} Fahrenheit is {} Celsius.", temp, converted);
    }
}

// Function to check if user wants to do another conversion
fn check_to_continue() -> bool {
    println!("\nPerform another conversion (yes/no):");
    let mut go_again = String::new();
    io::stdin()
        .read_line(&mut go_again)
        .expect("Failed to read line.");

    go_again.trim().eq_ignore_ascii_case("yes")
}

fn main() {
    println!("Welcome to Temperature Converter!");

    // Loop to allow continuous conversions
    loop {
        println!("---------------------------------");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius\n");

        let choice = get_choice();
        perform_conversion(choice);

        if !check_to_continue() {
            break;
        }
    }
}
