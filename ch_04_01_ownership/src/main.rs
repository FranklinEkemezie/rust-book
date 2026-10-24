
fn main() {

    // Ownership Rules

    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    // Variable scope

    {                           // s is not valid here; it's not yet declared
        let s = "hello";    // s is valid from this point forward  

        // do stuff with s
        println!("s: {s}");
    }                           // the scope is over; s is no longer valid

    // The `String` type

    // Unlike other data types we have seen such as integers, booleans, characters,
    // floating points, etc. which have fixed size, are stored in the stack and can
    // be trivially copied when another part of the code needs it, the `String` type 
    // is stored on the heap and a good example to understand how Rust knows how to
    // copy or when to clean up the data.

    // String type manages string data whose size cannot be determined at compile-time

    let s = String::from("hello");  // create a String from string literal

    println!("{s}");

    let mut s = String::from("hello"); // can also be made to be mutable

    s.push_str(", world!"); // mutate the string bt appending to it

    println!("{s}");

    // String type can be mutated but string literals cannot.
    // This is because string literals have fixed size, but String can grow

    // In order to be mutable:
    // - the required memory must be requested from the memory allocator at runtime:
    //      we do this using the `String::from()` function
    // - we need a way to return the allocated memory back to allocator when we are
    //      done: languages with garbage collector (GC) use it to keep track of and 
    //      clean up the memory that isn't being used anymore. Some other languages,
    //      require you to free it yourself. If we free it too early, we end up with
    //      an invalid variable. If we free it too late, we waste memory. If we do it
    //      more than once, we might end up with unexpected results. Thus, we need to
    //      pair exactly one `allocate` with exactly one `free` and it is very
    //      difficult to do this correctly. In Rust, memory is automatically returned
    //      once the variable that owns it goes out of scope.

    {
        let s = String::from("hello");  // s is valid from this point forward

        // do stuff with s
        println!("{s}");
    }                                           // scope is over; s is no longer valid

    // The point at which the scope is over becomes the natural point at which we can
    // return the memory our `String` needs to the allocator. When the variable goes
    // out of scope, Rust calls the `drop()` function which is where the memory used
    // to store the content of the string is released.
    // In C++, this is sometimes called the Resource Acquisition Is Initialisation
    // (RAII) pattern where resources used by a class is deallocated at the end of the
    // item's lifetime.

    // Variables and Data Interacting with Move

    // Multiple variables can interact with the same data in different ways in Rust.

    let x = 5;  // the value '5' is bound to variable x
    let y = x;  // the value stored in x is copied and bounded to variable y

    println!("x: {x}, y: {y}");

    // In the example above, we have a copy since the value has a known, fixed size
    // So, in reality, we have to 5 values pushed onto the stack. They are popped when
    // the variables storing them `x` and `y` go out of scope.

    // The example below is similar, but what happens is different

    let s1 = String::from("hello");
    let s2 = s1;

    // To understand what is happening: Think of a `String` type to contain various
    // data about the string it stores. As a compound type, it stores a pointer to the
    // actual content of the string which is stored in the heap; the length of the
    // string; and capacity which is the amount of memory we got from the allocator.
    // For performance reasons, the capacity is usually greater than the length of the
    // string so that we don't ask for memory frequently. Anyways, the important point
    // is that these information stored in the `String` i.e. pointer, length and capacity
    // are stored on the stack. This is because they have fixed length: a pointer is 
    // typically 4 bytes, while both length and capacity could be u32 integers. The string
    // data itself ("hello") is stored on the heap, and pointer points to it.
    // Now, when we assign `s1` to `s2`, these meta-data are copied (since they have fixed)
    // length, but the actual data in the heap is not copied, as it copying on the heap
    // is expensive. The result is that, both the `String` metadata of `s1` and `s2` have
    // pointers pointing to the same allocated (heap) memory. In other words, they both
    // own it. When `s1` goes out of scope, Rust automatically calls the `drop()` function
    // which cleans up the heap memory, i.e. frees the memory storing the "hello" data. But
    // `s2` will also go out of scope, and this will result in double free since they both
    // point to the same heap memory. To prevent this, Rust considers `s1` as no longer 
    // valid after the line `let s2 = s1;`. That way, the heap memory storing the string data
    // only has one owner. Rust does not need to free `s1` when it goes out of scope since it
    // is invalid.

    // println!("{s1}, world!");   // error: `s1` is now invalid
    println!("{s2}, world!");

    // This is kind of copy, where Rust copies only the meta-data (pointer, length, capacity)
    // without copying the actual string data is closer the term "shallow copy", as opposed to
    // "deep copy" where an object and all its underlying data are copied recursively and 
    // completely. Rust takes this an extra step by invalidating the first variable, and is
    // called a *move* (rather than just a shallow copy). We say `s1` was moved into `s2`.
    // The implication is that Rust never automatically creates "deep" copies of your data;
    // only known, fixed size data is copied, thus making automatic copying inexpensive.

    // Similarly, when you assign a new value to an existing variable, Rust calls `drop()` and
    // free the original value's memory immediately before binding the new value to the variable

    let mut s = String::from("hello");

    println!("s: {s}");

    // assigning a new value to it causes the previous string to go out of scope
    // and the memory allocated for "hello" is released
    s  = String::from("ahoy");

    println!("s: {s}");

    // If we really wish to deep-copy the heap data, we can use the `clone()` method.

    let s1 = String::from("hello");
    let s2 = s1.clone();    // here, a new heap memory is allocated separately

    println!("s1 = {s1}, s2 = {s2}");

    // As we have seen, stack-only data are copied (not moved) since doing so is
    // inexpensive, due to the fact they have known type at compile time.
    
    let x = 5;
    let y = x.clone();  // we could do this, though it's kind of pointless, and not any different from assiging `x` directly.

    println!("x = {x}, y = {y}");

    // In the code above, cloning integral (or stack-only) data is no different from 
    // copying. There is no distinction between "shallow-copy" and "deep-copy".
    // Rust has a special annotation called the `Copy` trait that can be placed
    // on types stored on the stack, just like integers. A type that implements this
    // `Copy` trait, will have their values copied, rather than moved, making the
    // variables of that type still valid after assignment to another variable.
    // However, we cannot a type with `Copy` if any of its part has implemented the
    // `Drop` trait. Generally, any scalar value or simple group of scalar values, and
    // values which do not require memory (or resource in general) allocation can
    // implement the `Copy` trait. This includes: all integer and floating-point types,
    // character, boolean types, tuple containing types implementing `Copy`

    // Ownership and Functions

    // Passing a variable to a function has similar semantics: it will it either
    // move or copy it depending on the type of variable. When it is moved, the variable
    // becomes invalid after the functions. This is illustrated below:

    let s = String::from("hello");      // `s` comes into scope

    takes_ownership(s);             // `s`'s value moves into the function...
        // ... and so is no longer valid here

    // println!("s: {s}"); error: `s` is no longer valid

    let x = 5;                              // x comes into scope

    makes_copy(x);                  // `x`'s value is copied to the function (as it is implements `Copy` trait)
        // it's okay to use `x` afterward.

    println!("after `makes_copy` function, x: {x}");    // okay!

    // Return values and scopes

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 = {s1}, s3 = {s3}");

    // In general, ownership of a variable follows the same pattern. When a value
    // is moved, the previous owner becomes invalid, and the owner is the sole owner.
    // When a value is copied instead, the original owner is still valid; the new
    // owner receives a fresh copy of the stack data. For function arguments, the
    // original (or previous) owner is the variable in the calling function, while
    // when returning the original owner is the variable in the called function.

    // While this works, taking ownership and returning ownership with every function is 
    // a bit tedious. We can use tuples to move a value and then return it with the result
    // as illustrated below 

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    // But this solution is not straightforward. We want to let a function to use
    // a value but not take ownership of it. Rust provides this as `references`.
    // See the "references" project for explanations and code examples.


}   // Here, x goes out of scope, nothing to free. `s` also goes out of scope, nothing special happens
    // since it is already invalid.

fn takes_ownership(some_string: String) {   // `some_string` comes into scope
    println!("{some_string}");              // `some_string` is still valid here
}                                           // `some_string` goes out of scope and `drop` is called. Backing memory is freed

fn makes_copy(some_integer: i32) {  // `some_integer` comes into scope
    println!("{some_integer}");     // `some_integer` is valid
}                                   // `some_integer` goes out of scope. Nothing special happens; no memory to free

fn gives_ownership() -> String {
    // this function returns a `String` type. This value is moved out into the 
    // function that calls it

    let some_string = String::from("yours");

    some_string     // `some_string` is returned and its value moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // `a_string` comes into scope

    a_string    // `a_string` is returned and its moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}