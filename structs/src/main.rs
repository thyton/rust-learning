struct User {
    active: bool,
    // Use String type rather than the &str string slice type.
    // This is a deliberate choice because we want each instance of this struct
    // to own all of its data and for that data to be valid for as long as
    // the entire struct is valid.
    // It's possible for structs to store references to data owned by
    // something else, but to do so requires the use of lifetimes,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    // 1. Defining and instantiating structs
    // Structs are like tuples which hold multiple related values of different
    // types. But structs have named fields, making them more flexible
    // since you don't need to rely on order to access values.

    // use mut to allow mutation on fields
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // 2. Using the field init shorthand
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");

    let user1 = User {
        active: true,
        username,   // uses field init shorthand because the username 
        email,      // and email variables have the same name as struct fields
        sign_in_count: 1,
    };

    // 2. Creating instances from other instances with struct update syntax
    // The ..user1 must come last to specify that any remaining fields should
    // get their values from the corresponding fields in user1, but we can
    // choose to specify values for as many fields as we want in any order,
    // regardless of the order of the fields in the struct’s definition.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // The struct update syntax uses = like an assignment; this is because it
    // moves the data.
    // We can no longer use user1 after creating user2 because the String
    // in the username field of user1 was moved into user2.

    // println!("{0}", user1.username); error!

    
    // If we had given user2 new String values for both email and username, and
    // only used the active and sign_in_count values from user1, then user1 
    // would still be valid after creating user2. Both active and sign_in_count
    // are types that implement the Copy trait, so the deep copy would
    // apply. We can also still use user1.email, because its value was not
    // moved out of user1.
    println!("{0}", user1.email);

    // 3. Using tuple structs without named field to create different types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unlike tuples, tuple structs require you to name the type of the struct
    // when you destructure them.
    let Point(x, y, z) = origin;
    let Color(a, b, c) = black;

    // 4. Unit-like structs without any fields
    struct AlwaysEqual; // similar to unit ()
    let subject = AlwaysEqual;
}
