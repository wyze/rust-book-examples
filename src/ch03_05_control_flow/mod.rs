pub fn run() {
    // If/Else If/Else
    let number = 3;

    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was something other than zero.");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number is not divisible by 4, 3, or 2.");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        // "six" here would throw an exception, each block must return the same type
        6
    };

    println!("The value of number is: {}", number);

    // Loop (infinite)
    #[allow(clippy::never_loop)]
    loop {
        println!("Again!");

        // We will exit the loop after the first iteration
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {}", result);

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!!!");

    // For loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // With a range
    // Will create an array like of [1, 2, 3], then reverse it and iterate over it
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Liftoff!!!");
}
