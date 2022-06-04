enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            // NOTE: Here 'Self' is an alias to 'VeryVerboseEnumOfThingsToDoWithNumbers'
            Self::Add => x + y,
            Self::Subtract => x + y,
        }
    }
}

fn main() {
    let x = Operations::Add;
}
