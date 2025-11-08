fn main() {
    // 3.1 Variables and Mutability
    // using mut to indicate the variable is mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // the second variable is shadowed until it itself is shadowed or
    // the scope ends
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    // it uses x from the same level scope
    println!("The value of x is: {x}");

    // notice tehe difference between using mut and let
    // here, it can assign a different value and a different type
    let spaces = "   ";
    let spaces = spaces.len();


    // this would throw a compile-erro because of the wrong type
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // 3.2 Data Types
    // every value in Rust is of a certain data type
    // Rush is a statically typed language. It must know the types
    // of all variables at compile-time

    // scalar
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // f32 and f64
    // architecture dependent	isize	usize

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation

    let t = true;
    let f: bool = false; // with explicit type annotation

    // compound
    // tuples
    // the types of the different values in the tuple don’t have to be the same
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}
