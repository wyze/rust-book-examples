pub fn run() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we
    // could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    // Following slice syntax is equal
    let slice = &s[0..2];
    let slice = &s[..2];

    let s = String::from("hello");
    let len = s.len();

    // Following slice syntax is equal
    let slice = &s[3..len];
    let slice = &s[3..];

    let s = String::from("hello");
    let len = s.len();

    // Following slice syntax is equal
    let slice = &s[0..len];
    let slice = &s[..];

    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear();

    // Following throws cannot borrow `s` exception
    // println!("The first word is: {}", word);

    // type of &str and is immutable reference
    let s = "Hello, world!";

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: /* &String -> */ &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
