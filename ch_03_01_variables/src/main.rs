use std::io;



fn main() {

    // immutability
    let mut x  = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours = {THREE_HOURS_IN_SECONDS} seconds.");

    // shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}");

    // error: cannot assign different type to a variable
    // let mut spaces = "    ";
    // spaces = spaces.len();

    // data types
    let guess: u32 = "42".parse().expect("Not a number!"); // must annotate type when it can't be inferred

    println!("Guess: {guess}");

    // scalar types

    // integers

    let num = 98_222;       // decimal
    println!("num {num}");

    let num = 0xff;         // hex
    println!("num {num}");

    let num = 0o77;         // octal
    println!("num {num}");

    let num = 0b1111_0000;  // binary
    println!("num {num}");

    let num = b'A';         // byte
    println!("NUm: {num}");

    let x = 250u8;
    let y = 10u8;
    
    // integer overflows

    println!("Wrap add: {}", x.wrapping_add(y));
    println!("Check add: {}", x.checked_add(y).get_or_insert_default()); // return 'None' if overflow occurs
    println!("Overflow add: {}, Overflow? {}", x.overflowing_add(y).0, x.overflowing_add(y).1);
    println!("Saturate add: {}", x.saturating_add(y));

    // floating-point types

    // Represented using IEEE-754 standard.

    let x = 2.0;    // default `f64`
    let y = 3.0f32;

    println!("x: {x}; y: {y}");

    println!("sum: {}", 5 + 10);
    println!("difference: {}", 95.5 - 4.3);
    println!("product: {}", 4 * 30);
    println!("quotient: {}", 56.7 / 32.2);
    println!("truncated: {}", -5 / 3);
    println!("remainder: {}", 43 % 5);

    // boolean `bool` => `true` or `false`

    // character `char`

    let c = 'z';
    println!("c: {c}");

    let z: char = '௹';
    println!("z: {z}");

    let heart_eyed_cat = '😻';
    println!("heart eyed cat: {heart_eyed_cat}");

    // compound types

    // tuple

    // fixed length; can contain multiple types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructure tuple using pattern matching
    let (x, y, z) = tup;

    println!("The value of x, y, z are: {x}, {y}, {z}");
    println!("The value of y is: {}", tup.1);   // get item directly using zero-based indexing

    // tuple without any values is called `unit` written and type-annotated
    // as `()`. It represents an empty value or return type. Expressions which
    // do not return any type implicitly return the unit value.

    // array

    // elements must have same type; fixed length
    // since size is known at compile-time, it is stored in the stack like
    // the other types we have seen so far.
    // unlike the `vector` type, it doesn't grow or shrink. A `vector` is
    // a collection type that can grow or shrink in size; its contents live
    // on the heap.

    let a: [i32; 5] = [1, 2, 3, 4, 5];  // an array of 5 `i32` elements
    println!("a: {:?}", a);

    let a = [3; 5]; // an array containing 3 five times
    println!("a: {:?}", a);

    let a = [5, 4, 3, 2, 1];
    println!("a[2]: {}", a[2]); // access array elements using zero-based index

    // invalid array element access

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Incex entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}