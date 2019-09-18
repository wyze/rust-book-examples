use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! vec2 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*

            temp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes;

pub fn run() {
    let v = vec2![1, 2, 3];

    println!("v = {:?}", v);

    Pancakes::hello_macro();
}
