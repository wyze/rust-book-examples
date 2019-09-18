type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

pub fn run() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}

fn takes_long_type(f: Thunk) {}
fn returns_long_type() -> Thunk {
    Box::new(|| println!("ok"))
}

// Never type
fn bar() -> ! {
    // `panic` has return type of !
    // panic!();

    loop {
        print!("loopin' ");
    }
}

fn generic<T>(t: T) {}

// Above is shorthand for
fn generic2<T: Sized>(t: T) {}

// ? indicates T may or may not be sized
fn generic3<T: ?Sized>(t: &T) {}
