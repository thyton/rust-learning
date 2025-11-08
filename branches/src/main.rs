fn main() {
    // conditions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 3 and less than 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // repetitions
    // loop
    let mut cnt = 0;

    let result = loop {
        cnt += 1;
        if cnt == 10 {
            // pass the result out the loop
            break cnt * 2;
        }
    };

    println!{"The result is {result}"};

    // loop labels to disambiguate between multiple loops
    let mut cnt = 0;
    'counting_up: loop {
        println!{"count: {cnt}"};
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            // break the loop of counting_up label
            if cnt == 2 { break 'counting_up;}
            remaining -= 1;
        }

        cnt += 1;
    }

    println!("End count = {cnt}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [1, 2, 3, 4, 5];

    for elem in a {
        println!("the value is: {elem}");
    }

    for num in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");

    let x = fibonacci(6);
    println!("Fibonacci(6) is {x}");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}