// Opt-in to functionality to print out debugging information
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are functions defined within the context of a struct,
// (or an enum or a trai object) with self as their first parameter
impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associated functions
// All functions defined within an impl block are called associated functions
// because they're associated with the type name after the impl.
// They can be defined without self as the first parameter.
// String::from function is defined on the String type
impl Rectangle {
    // The Self keywords in the return type and in the body of the function
    // are aliases for the type that appears after the impl keyword,
    // which in this case is Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels.",
        area(&rect1));

    // Rust has a feature called automatic referencing and dereferencing.
    // For calling methods, the self pointer is auto-computed. 

    println!(
        "The area of the rectangle is {} {} square pixels.",
        rect1.area(), &rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // The println! macro by default, use formatting known as Display
    // Primitive types implement Display by default.
    // The specifier tells println! to use an output format called Debug
    println!("rect1 is {rect1:?}");

    // We can put dbg! around the expression 30 * scale and, because dbg! 
    // returns ownership of the expression’s value, the width field will get
    // the same value as if we didn’t have the dbg! call there.
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}