fn main() {
    println!("Hello, world!");
    
    another_function();

    print_labeled_measurement(5, 'h');

    // statements
    // creating a variable and assigning a value to it with the let 
    // words is a statement
    // a main function declaration is a statement
    let y = 6;

    // can't do the following in Rust since the Rust assignment return 
    // doesn't return the value of the assignment
    // let x = y = 6; 

    // expressions evaluate to a value
    // calling a function/macro is an expression

    // a new scope block created with curly braces is an expression

    let y = {
        let x = 3;
        // Note: no semicolon at the end
        // adding semicolon turns the expression to a function which won't
        // return a value, which is expressed by `()`, the unit type
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}