pub fn run() {
    // Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable reference
    let mut s = String::from("hello");

    change(&mut s);

    // Only 1 mutable reference per scope

    let r1 = &mut s;
    let r2 = &mut s;
    let r3 = &s;

    // If uncommented, cannot borrow s more than 1 time exception
    // println!("{}, {}", r1, r2);

    // If uncommented, cannot borrow `s` as mutable as it is also borrowed as immutable
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;

    println!("{}", r3);

    // Dangling references
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of
  // what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {    // dangle returns a reference to a String
//     let s = String::from("hello");  // s is a new String

//     &s  // we return a reference to the String, s
// }   // Here, s goes out of scope, and is dropped. It's memory goes away. Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
