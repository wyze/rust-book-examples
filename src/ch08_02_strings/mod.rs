pub fn run() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // Also on string literals directly
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("Hello");
    let hello = String::from("你好");
    let hello = String::from("Olá");

    let mut s = String::from("foo");

    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println!("s2 is {}", s2);

    let mut s = String::from("lo");

    s.push('l');

    println!("The value of s is: {}", s);

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("The value of s3 is: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("The value of s is: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("The value of s is: {}", s);

    let s1 = String::from("hello");
    // let h = s1[0]; // Cannot access characters by index

    let len = String::from("Hola").len();

    println!("The value of len is: {}", len);

    let len = String::from("Здравствуйте").len();

    println!("The value of len is: {}", len);

    // String slices
    let hello = "Здравствуйте";
    let s = &hello[..4];

    println!("The value of s is: {}", s);

    // Below will panic with byte index 1 is not a char boundary
    // let s = &hello[..1];

    // Iteration
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
