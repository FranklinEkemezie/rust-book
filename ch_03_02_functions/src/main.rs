
fn main() {

    println!("Hello, world!");

    another_function(5);
    print_labelled_measurement(5, 'h');

    // statements and expressions

    // statements are instructions that perform some action and do not return a value.
    // expressions evaluate to a resultant value.

    let _y = 6; // this is a statement
    // let x = (let y = 6); // you can't do this, `let` statement does not return a value.

    // expressions are usually part of a statement
    // - the value assigned to a variable is an expression
    // - function or macro call is an expression
    // - new scope block with curly brackets is also an expression.

    // below the expression { ... } evaluates to 4 and is bound to 4.
    let y = {
        let x = 3;
        x + 1   // no semicolon, returns a value; with semicolon it becomes a statement and does not return a value
    };

    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x is: {x}");

    // comments: use // or /* */

    // control flow: see `branches`


}

fn another_function(x: i32) {
    println!("Another function");
    println!("The value of x is: {x}");
} // function defintions are statements

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn five() -> i32 { // declare function return type after `->`
    5 // return the value from the expression '5'
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // returns `()` the unit type; error: returned type different from annotated return type
}
