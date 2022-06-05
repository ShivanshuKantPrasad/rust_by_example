use std::convert::From;

#[derive(Debug)]
struct Number {
    _value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { _value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {num:?}");
}
