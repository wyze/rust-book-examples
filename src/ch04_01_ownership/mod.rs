pub fn run() {
    // Variable scope
    {
        // s is not valid here, it hasn't been declared yet
        let s = "Hello"; // s is valid from this point forward

        // Do stuff with s
        println!("The value of s is: {}", s);
    } // This scope is now over, and s is no longer valid

    // String type
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // Move data
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    // Below will throw an exception, invalid after move
    // println!("{}, world!", s1);

    // Clone data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Copy data (Stack only)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Functions
    {
        let s = String::from("hello"); // s comes into scope

        take_ownership(s); // s's value moves into the function..
                           // .. and so is no longer valid here

        // The following throws exception, borrow of moved value `s`
        // println!("The value of s is: {}", s);

        let x = 5; // x comes into scope

        makes_copy(5); // x would move into the function,
                       // but i32 is Copy, so it's ok to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved,
      // nothing special happens.

    // Return values
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } // Here, s3 goes out of scome and is dropped. s2 goes out of scope but
      // was moved, so nothing happens. s1 goes out of scope and is dropped.

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn take_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return
    // value into the function that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling func
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of the string

    (s, length)
}
