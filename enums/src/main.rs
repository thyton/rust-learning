// enums give you a way of saying a value is one of a possible set of values
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

// Rather than using an enum inside a struct, we can put data directly into
// each enum variant. We can any kind of data inside an enum variant:
// strings, numeric types, or structs,
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

// Enum definition is similar to defining different kinds of struct definitions,
// except the enum doesn’t use the struct keyword and all the variants are
// grouped together under the Message type
// If we used the different structs, each of which has its own type,
// we couldn’t as easily define a function to take any of these kinds
// of messages as we could with the Message enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enum methods
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// The Option Enum and its advantages over null values
// The Option type (from the stardard library) encodes the very common scenario 
// in which a value could be something or it could be nothing.
// enum Option<T> {
//     None,
//     Some(T),
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // matches are exhaustive. The arms' patterns must cover all possibilities
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Another useful feature of match arms is that they can bind to
        // the parts of the values that match the pattern
        // This is how we can extract values out of enum variants
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // Option<T> and T (where T can be any type) are different types,
    // the compiler won’t let us use an Option<T> value as if it were
    // definitely a valid value.
    // let sum = x + y; error!

    // We have to convert an Option<T> to a T before you can perform
    // T operations with it. Generally, this helps catch one of the most
    // common issues with null: assuming that something isn’t null when it
    // actually is.

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Catch-all patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        other => println!("move player {}", other),
    }

    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        _ => (),
    }

    // Concise control flow with if let and let else
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!")
    } else {
        count += 1;
    }

    // let .. else syntax
    let coin = Coin::Quarter(UsState::Alaska);
    let Coin::Quarter(state) = coin else {
        println!("not a quarter");
        return;
    };

    match state {
        UsState::Alabama => println!("{state:?} is pretty old, for America!"),
        _ => println!("{state:?} is relatively new."),
    }
}
