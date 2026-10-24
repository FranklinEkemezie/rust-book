use std::io;

const MIN_OPTION: u8 = 0;
const MAX_OPTION: u8 = 2;

fn main() {

    println!("Welcome to Temperature Converter!");

    loop {
        let option = get_user_option();

        if option == 0 { return; }

        let temperature = get_temperature();

        if option == 1 {
            println!("{temperature}°C => {}°F", to_fahrenheit(temperature));
        } else if option == 2 {
            println!("{temperature}°F => {}°C", to_celsius(temperature));
        }

        println!();
    }


}

fn show_option() {

    println!("1> Fahrenheit to Celsuis");
    println!("2> Celsuis to Fahrenheit");
    println!("0> Exit");
}

fn get_input(label: String) -> String {
    println!("{label}: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get user input");

    return input;
}

fn get_user_option() -> u8 {

    show_option();

    let option: u8 = get_input(String::from("Please choose an option"))
        .trim()
        .parse()
        .expect("Please enter a valid option");

    assert!(option >= MIN_OPTION && option <= MAX_OPTION, "Please enter a valid option");

    return option;
}

fn get_temperature() -> f64 {

    let temperature: f64 = get_input(String::from("Enter temperature"))
        .trim()
        .parse()
        .expect("Invalid temperature");

    return temperature;
}

fn to_fahrenheit(celsuis: f64) -> f64 { celsuis * (9.0 / 5.0) + 32.0 }

fn to_celsius(fahrenheit: f64) -> f64 { (fahrenheit - 32.0) * (5.0 / 9.0) }