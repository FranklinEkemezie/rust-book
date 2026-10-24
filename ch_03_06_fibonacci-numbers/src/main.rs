use std::io;


fn main() {
    
    let mut n = String::new();

    println!("Enter 'n': ");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read user input");

    let n: u128 = n.trim().parse().expect("Invalid input");

    println!("{n}{} fibonacci number is {}",  n_cardinal(n), nth_fibonacci_number(n));
}


fn nth_fibonacci_number(n: u128) -> u128 {
    if n <= 1 { return n; }
    
    nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2)
}

fn n_cardinal(n: u128) -> String {
    let n = n % 100;

    if n >= 11 && n <= 20 { return String::from("th"); }

    match n % 10 {
        1   => String::from("st"),
        2   => String::from("nd"),
        3   => String::from("rd"),
        _   => String::from("th")
    }
}