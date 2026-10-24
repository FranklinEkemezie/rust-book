fn main() {

    // The `Slice` Type

    // Slices is a kind of reference (so it does not have ownership) to a 
    // contiguous sequence of elements in a collection.
    // We'll demonstrate this with a small problem.
    // write a function that takes a string of words separated by spaces 
    // and returns the first word it finds in that string. If the 
    // function doesn’t find a space in the string, the whole string must 
    // be one word, so the entire string should be returned.


    let mut s  = String::from("hello world");

    let word = first_word(&s); // `word` is `5` i.e. index of end of first word

    println!("index of end of first word: {word}");

    s.clear(); // empties the `String` making it equal to ""

    // `word` stil has the value `5` even though `s` has changed
    // using it to grab or work with the first word of `s` leads to a bug

    // we can instead using "string slices" to refer to a contiguous sequence
    // of the elements of a `String`

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello = {hello}; world = {world}");

    // Internally, the `slice` data structure stores the starting position and
    // length of the slice. Thus, `world` for example, would store a pointer to
    // the byte at `6` and length value of `5`. For the slice &s[a..b], the index
    // `b` is not included i.e. the slice is really [a, b)

    let s = String::from("hello");

    let slice = &s[..2]; // same as slice from 0 to 2
    println!("slice = {slice}");

    let slice = &s[3..];    // same as slice from 0 to s.len()
    println!("slice = {slice}");

    let slice = &s[..];     // slice of entire string
    println!("slice = {slice}");

    // NB: String slice range indices must occur at valid UTF-8 character
    // boundaries. If you attempt to create a string slice in the middle of
    // a multibyte character, your program will exit with an error

    // Now we can call the new `first_word()` function

    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("the first word is: {word}");

    // call to `clear()` method fails to compile
    // `clear()` requires a mutable reference to `s` in order to, you know,
    // clear it. But the immutable reference `word` is still in scope as it
    // it being used afterwards. This code would work, if we didn't use `word`
    // after calling `clear()`.
    // This guarantees that our string slice is valid.
    s.clear();

    // println!("the first word is: {word}");

    // String Literals as Slices

    // Recall that string literals as being stored in the binary. That is,
    // if you could open the generated binary and go through it, you would
    // "see" the string literal in it. In Rust, the type of a  string literal
    // is `&str` i.e. it is an immutable reference specifically it is a string
    // slice.

    let s = "Hello, world!";
    println!("s: {s}");

    // When accepting a string as parameter, it is more convenient to accept a
    // string slice as it allows both the `String` type and `&str` (string slice)
    // type to be passed. The compiler uses "implicit deref coercions" to "convert"
    // the `String` type to a string slice.

    let my_string = String::from("hello world");

    println!("{}", first_word(&my_string[0..6]));
    println!("{}", first_word(&my_string[..]));
    println!("{}", first_word(&my_string));

    let my_string_literal = "hello world";

    println!("{}", first_word(&my_string_literal[0..6]));
    println!("{}", first_word(&my_string_literal[..]));
    println!("{}", first_word(my_string_literal));

    // Other Slices

    // There are other slices, aside from string slices. For example,
    // we have the array slice shown below

    let a = [1, 2, 3, 4, 5];

    // the `slice` below is a reference to contiguous elements whose
    // types are `i32`
    let slice = &a[1..3];

    // below, we create our own array slice using  the
    // `&[x1, x2, x3, ..., xn]` syntax
    assert_eq!(slice, &[2, 3]); // check we got the right slice

    println!("{:?}", slice);

    // Next in Chapter 5, we talk about grouping pieces of data
    // together in a `struct`.

}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' { 
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}