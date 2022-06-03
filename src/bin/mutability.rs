fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    print!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //_immutable_binding += 1;
    // ERROR Cannot change value of an immutable object
}
