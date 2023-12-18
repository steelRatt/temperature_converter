use std::io;

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

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn get_choice() -> f64 {
    loop {
        let choice = get_input("Enter (1 or 2):");

        if choice == 1.0 || choice == 2.0 {
            return choice;
        } else {
            println!("Invalid choice.");
        }
    }
}

fn perform_conversion(choice: f64) {
    if choice == 1.0 {
        let temp = get_input("Enter temperature in C:");
        let converted = celsius_to_fahrenheit(temp);
        println!("{} Celsius is {} Fahrenheit.", temp, converted);
    } else if choice == 2.0 {
        let temp = get_input("Enter temperature in F:");
        let converted = fahrenheit_to_celsius(temp);
        println!("{} Fahrenheit is {} Celsius.", temp, converted);
    }
}

fn check_to_continue() -> bool {
    println!("Perform another conversion (yes/no):");
    let mut go_again = String::new();
    io::stdin()
        .read_line(&mut go_again)
        .expect("Failed to read line.");

    go_again.trim().eq_ignore_ascii_case("yes")
}

fn main() {
    println!("Welcome to Temperature Converter!");

    loop {
        println!("---------------------------------");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");

        let choice = get_choice();
        perform_conversion(choice);

        if !check_to_continue() {
            break;
        }
    }
}
