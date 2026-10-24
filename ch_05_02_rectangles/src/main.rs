

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        // `&self` parameter type is short for `self: &Self`.
        // In an `impl` block, it is an alias of the current of the `impl` block.
        // We use `&` to borrow the object immutably, so that the caller still owns it.
        // Using just `self` as parameter (i.e. taking ownership) is rare, and is
        // typically used when the method transforms the object to something else, and
        // we want to prevent the caller from using the old version

        self.width * self.height
    }

    fn width(&self) -> bool {
        // it's okay to have a method with same name as one of the struct fields.
        // Rust knows tha `rect.width` refers to `width` struct field, 
        // while `rect.width()` is the method. Having a method with same name as
        // as a field is commonly used to implement "getters" - methods with same
        // name as a field and return the value of that field (typically with extra
        // sanity checks). This is useful as you can make the underlying implementation
        // private (hide the fields), and expose "getters" and other methods to users of
        // the code manipulate the struct's data, instead of allowing them to manipulate
        // it directly.

        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define associated functions (functions within an `impl` block) that do not
    // take `self` as their first parameters (and thus are not methods - some languages call
    // it "static methods"). An example is `String::from()`. They can also be used as
    // constructors to return a new instance of the struct, and often named `new()`
    // Below we have the associated function to create a rectangle with equal height and width

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

}

// We can have multiple `impl` blocks, even though it's kind of pointless here
// Multiple `impl` blocks for the same type can be useful when discussing generic types and traits.

impl Rectangle {
    
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {

    // V1

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // V2

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // )

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // you can't just print the struct. Primitive types by default implement
    // `Display` trait and simply prints them the way they are, since
    // there's only one way to print a primitive type like `1` or `true`.
    // For user defined types, you can have to specify that.

    // println!("rect1 is {rect1}");   // fails, suggests using `{:?}` or `{:#?}`
    // println!("rect1 is {rect1:?}"); // still fails, now it says it needs to implement `Debug` trait.

    // `Debug` trait allow us to print structs in a useful way for developers.
    // We can include functionality to print out debugging information by add
    // the outer attribute `#[derive(Debug)]` as the error help message suggests.

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}"); // pretty print

    // We can also print using the `Debug` format by using `dbg!()` macro.
    // It takes ownership of the data (as opposed to `println!()` which takes a
    // reference), prints the file, line number of the macro call and resultant
    // value of that expression, and returns ownership of the value. This macro
    // prints to the "standard error console stream (`stderr`)" as opposed to
    // `println!()` which prints to the "standard output console stream (`stdout`)".
    // We'll tak bout `stderr` and `stdout` in Chapter 12.

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("--- Method Syntax");
    // Methods are function which are defined in the context of (i.e. tied to)
    // an struct (or an enum, or a trait object). Their first parameter is `self`
    // which represents the instance of the struct the method is being called on.
    // Struct methods are defined in a `impl` (implementation) block to specify
    // the context i.e. the struct type the methods apply to.


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // In programming languages like C and C++, where there are two ways
    // to call a method depending on whether the object is a pointer or not: `.`
    // is used to call the method on the object directly while `->` is used 
    // on the reference or pointer. In Rust, you don't have the `->` operator;
    // you need to dereference the pointer. However, Rust has a feature called
    // "automatic referencing and dereferencing" where it automatically adds in
    // `&`, `&mut`, or `*` as appropriate so that the object matches the signature
    // of the method

    println!("--- Methods with More Parameters");

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("--- Associated Functions");
    
    let sq = Rectangle::square(3);
    println!("sq: {sq:#?}");

    println!("--- Multiple `impl` blocks");

    dbg!(Rectangle::new(30, 15));

    
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

