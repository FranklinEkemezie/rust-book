
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple struct: Tuples which now have a distinct type
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct: can be useful when you need to implement a
// trait on some type but you don't have any data to store in the type itself
struct AlwaysEqual;

// Storing references require lifetimes (Not covered yet)
// struct SomeUser {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64
// }

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    println!("User1 Email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("User1 Email: {}", user1.email);

    let user2 = build_user(String::from("johndoe@example.com"), String::from("johndoe32"));

    println!("Username: {}; Is active? {}; Sign-in count: {}", user2.username, user2.active, user2.sign_in_count);

    let user2  = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };

    println!("Username: {}; Email: {}", user2.username, user2.email);

    // We can also create `user2` from another struct using the `..` update syntax
    let user1 = build_user(String::from("someone@example.com"), String::from("someone123"));
    println!("User 1 => Username: {}; Email: {}", user1.username, user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // must come last, and uses the other fields as they are
    };

    println!("User 2 => Username: {}; Email: {}", user2.username, user2.email);

    // The `User` struct contains the `String` type that implements the `Move` trait.
    // This means that after using `user1` data to create `user2`, we cannot use
    // `user1` as a whole again as one (or more) of its data (implementing the `Move`
    // trait) have been moved. We can however still use fields that implement the
    // `Copy` trait, such as `active` and `sign_in_count`. We can also use fields
    // that implement `Move` trait that are not used in the creating `user2` since
    // they are not moved. For example, `email` field in the previous example.

    // println!("User 1 => Username:   {}", user1.username);   // fail to compile: `user1.username` is moved
    println!("User 1 => Email:      {}", user1.email);      // ok!: not moved.


    println!("--- Using Tuple structs");

    let black = Color(0, 0, 0);
    println!("R: {}; G: {}; B: {}", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);
    println!("x: {}; y: {}; z: {}", origin.0, origin.1, origin.2);

    println!("--- You must name the tuple struct type when destructing");

    let Point(x, y, z) = origin;
    println!("(x, y, z) => ({x}, {y}, {z})");

    println!("--- Unit-like structs - structs without any fields");
    let _subject = AlwaysEqual;

    println!("--- Ownership of struct data");
    // Currently, our `User` struct uses the owned `String` type rather than
    // `&str` string slice type as we want each instance of the struct to own
    // all of its data. The data it owns is valid for as long as the entire struct
    // is valid. To store references in struct require that we specify the "lifetime"
    // of the struct. *Lifetimes* are discussed in Chapter 10. Since storing a
    // reference means that that data is not owned by the struct, lifetimes helps
    // us to ensure that the data referenced by a struct is valid for as long as
    // the struct is. Thus, the definition of the `SomeUser` struct would fail:

    // let someUser = SomeUser {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1
    // };


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1 
    }
}