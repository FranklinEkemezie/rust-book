

fn main() {
    let number = 7;

    if number < 5 {
    // if number { // condition must be `bool`, do the below instead
    // if number != 0 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number { // condition must be `bool`, do the below instead
    if number != 0 {
        println!("number was three");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2");
    }

    // in rust, `if` is an expression and can be assigned to a variable

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // error: arms result in different type
    // let number = if condition { 5 }; // does not work too, arms result in integer or () type

    println!("The value of number is: {number}");

}
