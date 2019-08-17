#[derive(Debug)]
enum UsState {
    Illinois,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn run() {
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Same as above ^^
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Dime;

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Same as above ^^
    let mut count = 0;
    let coin = Coin::Nickel;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
