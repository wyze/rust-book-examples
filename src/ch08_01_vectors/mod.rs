enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn run() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        println!("The value of v is: {:?}", v);
    } // <- v goes out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // Below will panic with index out of bounds error
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // Below will panic with cannot borrow `v` as mutable because also borrowed
    // v.push(6);

    println!("The first element is: {}", first);

    // Iterating over values
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
