fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50;
        // ERROR _mutable_integer is frozen in this scope
    }

    _mutable_integer = 3;
    // Ok! _mutable_integer is not frozen in this scope
}
