
fn main() {
    println!("Hello, world!");

    for day in 1..=12 {
        println!("On the {day}{} of Christmas, my true love said to me...", 
            n_cardinal(day));
    }
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
