pub fn run() {
    // Number literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // Floating-Points
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 38;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f: bool = false;

    // Character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Array
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Specify type like the following: [data type; length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Following will create an array length of 5 with value of 3
    // Same as writing: let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    let index = 10;
    // Following would throw index out of bound exception
    // let element = a[index];
}
