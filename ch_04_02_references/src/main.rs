fn main() {

    // References allow another part of our code e.g. a function to use a
    // value without having to transfer ownership.
    // The `calculate_length()` from "ownership" project can be rewritten as

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // pass a reference to `s1`
    
    println!("The length of '{s1}' is {len}.");


    //

    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s; // error: we can have only one mutable reference

    // println!("{r1}, {r2}");
    println!("{r1}");

    // Having only one mutable references lets Rust prevent data races at compile
    // time. A data race is similar to a race condition and happens when these
    // three behaviours occur:
    // - two or more pointers access the same data at the same time.
    // - at least one of the pointers is being used to write to the data.
    // - there's no mechanism being used to synchronise access to the data.

    // Data races cause undefined behaviour, and can be difficult to diagnose and
    // fix at runtime. Rust prevents this problem by refusiong to compile code with
    // data races..

    // We can use curly braces to create a new scope, allowing for multiple mutable
    // references, just not simultaneous ones:

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 = {r1}");
    }

    let r2 = &mut s;
    println!("r1 = {r1}; r2 = {r2}");


    // You cannot also have a mutable reference while having an immutable
    // one to the same value.

    // let mut s = String::from("hello");
    let s = String::from("hello");


    let r1 = &s;    // no problem
    let r2 = &s;    // no problem

    println!("r1 = {r1}; r2 = {r2}");

    // let r3 = &mut s;    // BIG PROBLEM: cannot borrow as mutable, already borrowed as immutable

    // println!("r1 = {r1}; r2 = {r2}");

    // This is because the code using immutable references does not expect the 
    // the value to suddenly change, which would be the case when a mutable
    // reference exists and is used to change the value. We can have multiple
    // mutable references since there is no need to worry about one reference
    // changing the value.

    // A reference's scope starts from where it is introduced and continues to the
    // last time that reference is used. This means we create a mutable reference
    // even we have some existing mutable reference as long as we don't use the
    // first one anywhere else, after the second one is created.


    let mut s = String::from("hello");

    let r1 = &s;    // no problem
    let r2 = &s;    // no problem

    println!("{r1} and {r2}");

    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s;    // no problem

    println!("{r3}");

    // Simply put, the scopes of mutable references must not overlap!

    // In Rust, the compiler guarantees that references will never be dangling
    // reference, i.e. the compiler ensures that the data a reference references
    // will not go out of scope before the reference to the data does.

    let reference_to_nothing = dangle();

    println!("reference_to_nothing: {reference_to_nothing}");

    // In conclusion, the rule of references can be summarised as:
    // - at any given time, you can have either one mutable reference,
    //      or any number of immutable references.
    // - references must always be valid.

    // Next, we look at a different kind of reference: slices.


}

fn calculate_length(s: &String) -> usize {
    // parameter `s` is a reference to a `String`
    // it allows us to "borrow" a `String` without owning it

    s.len()
}

fn change(some_string: &mut String) {
    // we must make the `some_string` reference mutable

    some_string.push_str(", world");
}

// fn dangle() -> &String {

//     let s = String::from("hello");  // `s` is a new String

//     &s  // we return a reference to the String
// }   // `s` goes out of scope and is dropped, so its memory goes away. 
    // Dangerous! Does not compile!! 
    // See below

fn dangle() -> String {
    let s = String::from("hello");

    s // ownership is returned to caller i.e. moved out to caller
}