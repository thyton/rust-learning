fn main() {
    // 1. Ownership rules

    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // string literals are immutable
    // The contents are known at compile time, so the text is hardcoded directly
    // into the final executable.

    // Rust String is mutable. It manages data allocated on the heap
    // is able to store an amount of text that is unknown at compile time
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator
    // when we’re done with our String.
    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

    // In Rust, the memory is automatically returned once the variable
    // that owns it goes out of scope, Rust calls a special function called drop
    // It’s where the author of String can put the code to return the memory.
    // Rust calls drop automatically at the closing curly bracket.

    // 2. Variables and data interating with move

    // bind 5 to x and then make a copy of the value in x and bind to y
    // two values of 5 are pushed onto the stack
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    // To avoid a double free error, Rust considers s1 as no longer valid
    // In Rust, we would say s1 was move into s2 -> shallow copy

    // Rust doesn't need to free anything when s1 goes out of scope
    // Freeing memory twice can lead to memory corruption

    // Rust will never automatically create "deep" copies of your data

    // println!("{s1}, world!"); will error "value borrowed here after move"

    // 3. Scope and assignment

    // we initially declare a variable and bind it to a String with "hello" value
    let mut s = String::from("hello");
    s = String::from("ahoy");
    // At this point, nothing is referring to the original String on the heap.
    // Thus, it the String goes out of scope, the memory will be freed right away
    println!("{s}, world!");

    // 4. Variables and data interacting with clone

    let s1 = String::from("hello");
    // explicit deep copy
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // 5. Stack-only data: copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    // Though this code seems to contradict what we learned: we don't have to
    // call clone, but x is still valid and wasn't moved into y.

    // The reason is types like integers have a known size at
    // compile time and are stored entirely on the stack, so copies
    // the actual values are quick to make.

    // There is no diferrence between deep and shallow copying here.

    // Rust has a special annotation called the Copy trait that we can place
    // on types that are stored on the stack, as integers are.
    // If a type implements Copy trait, variables that use it don't move,
    // but rather are deep-copied, making them still valid after assignment.

    // Rust won't let us annotate a type with Copy if the type, or any of its
    // parts, has implemented the Drop trait. It will result in a compile-time
    // error when we add the Copy annotation to the type to for something special
    // when its value goes out of scope.

    // Types that implement the Copy trait

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy.
    // For example, (i32, i32) implements Copy, but (i32, String) does not.

    // 6. Ownership and functions
    // Passing a variable to a function will move or copy, as assignment does.
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // Because i32 implements the Copy trait,
                                        // x does NOT move into the function,
                                        // so it's okay to use x afterward.
    } // Here, x goes out of scope, then s. However, because s's value was moved,
      // nothing special happens.

    // 7. Return values and scope
    // Returning values can transfor ownership.

    {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1

        let s2 = String::from("hello");     // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    // The ownership of a variable follows the same pattern: assigning a value
    // to another variable moves it. The data of a variable that goes out scope
    // will be cleaned up by drop unless ownership of the data has been moved
    // to another variable

    // If we want to let a function use a value but not take ownership, we
    // pass back, in addition to any data resulting from the function that we
    // want
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }

    // But this is too much ceremony. Luckily, Rush has a feature for using
    // a value WITHOUT TRANSFERRING ownership, called references

    // 8. References and borrowing
    // A reference is like a pointer in that it’s an address we can follow to
    // access the data stored at that address; that data is owned by some 
    // other variable. Unlike a pointer, a reference is guaranteed to point to
    // a valid value of a particular type for the life of that reference.
    {
        let s1 = String::from("hello");

        let len = calculate_length_ref(&s1);

        println!("The length of '{s1}' is {len}.");
    }

    // We call the action of creating a reference borrowing.
    // Just as variables are immutable by default, so are references. 
    // We’re not allowed to modify something we have a reference to.

    // 9. Mutuable references
    // We use a mutuable reference when we want to modify a borrowed value
    {
        let mut s = String::from("hello");

        change(&mut s);
    }

    // Mutable references have ONE BIG RESTRICTION:
    // if you have a mutable reference to a value, you can have no other
    // references to that value.
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // Uncomment to see the compile-time error of borrowing s
        // as mutable more than once
        // let r2 = &mut s;

        // println!("{r1}, {r2}");
    }

    // The restriction benefit: Prevent data races at compile time
    // Similar to a race condtion, a data raceh happens when
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    // Data races cause undefined behavior and can be difficult to diagnose
    // fix at runtime; Rust prevents this by refusing to compile code with
    // data races.

    // As always, we can use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with
      // no problems.

    let r2 = &mut s;

    // we CANNOT have a mutable reference while we have an immutable one to
    // the same value
    // Users of an immutable reference don't expect the value to suddenly change
    // from under them!
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        // let r3 = &mut s; // BIG PROBLEM

        // println!("{r1}, {r2}, and {r3}");        
    }

    // Thiw will compile because the scopes of the immutable references r1 and
    // r2 end after println! where they are last used, which is before the
    // mutable reference r3 is created  
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // Variables r1 and r2 will not be used after this point.

        let r3 = &mut s; // no problem
        println!("{r3}");
    }

    // 10. Dangling references
    // a dangling pointer is a pointer to a previously freed memory
    // that may have been given to someone else.

    // Rust prevents dangling references at compile-time, ensuring
    // references must be valid. i.e. the data will not go out of scope 
    // before the reference to the data does
    {
        let reference_to_nothing = dangle();

        // The following results in a compile-time error
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }
        fn dangle() -> String {
            let s = String::from("hello");
            s
        }
    }

    // 11. The slice type
    // Slice is a kind of reference, so it does not have ownership
    // It's a reference to a contiguous sequence of elements in a collection
    {
        let s = String::from("hello world");

        // Internally, the slice data structure stores the starting position
        // and the length of the slice, which corresponds to ending_index
        // minus starting_index
        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{hello} {world}")
    }

    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        // this would error as clear() uses a mutable borrow
        // s.clear(); // error!

        println!("the first word is: {word}");
    }

    // 12. String literals as slices
    {
        // string literals are stored inside the binary
        // the type of s is a slice pointing to that specific point
        // in the binary. Hence, string literals are immutable;
        // &str in an immutable reference
        let s = "Hello, world!";
    }

    // 13. String slices as parameters
    // If we have a string slice, we can pass that directly. If we have
    // a String, we can pass a slice of the String or a reference to the String.
    // This flexibility takes advantage of deref coercions.
    {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole.
        let word = first_word_str(&my_string[0..6]);
        // `first_word` also works on references to `String`s, which are
        // equivalent to whole slices of `String`s.
        let word = first_word_str(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or
        // whole.
        let word = first_word_str(&my_string_literal[0..6]);
        let word = first_word_str(&my_string_literal[..]);
    }

    // 14. Other slices
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

// gives_ownership will move its return value into the function
// that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// takes_and_gives_back takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {// s is a reference to a String
    // When functions have references as parameters instead of the actual values,
    // we won’t need to return the values in order to give back ownership,
    // because we never had ownership.
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
