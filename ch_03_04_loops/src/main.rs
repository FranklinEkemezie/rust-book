
fn main() {

    // infinite loop: break out using Ctrl+C
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels

    // `break` and `continue` apply to the innermost loop when you have loops within loops.
    // Use loop labels to disambiguate between multiple loops.

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // `while` loops

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // `for` loop: loop through a collection

    // without `for` loop: using `while` loop

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("--- without `for` loop");
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // with `for` loop:

    println!("--- with `for` loop");
    for element in a {
        println!("the value is: {element}");
    }

    // it is common for the "countdown" example above to be done using
    // a `for` loop + `Range`
    // we generate the range of numbers we want and loop over them using `for` loop

    println!("--- using range for countdown: ");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


}
