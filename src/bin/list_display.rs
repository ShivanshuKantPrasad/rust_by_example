use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    println!("{}", List(vec![90, 24, 35, 67]));
    println!("{:?}", List(vec![90, 24, 35, 67]));
    println!("{:#?}", List(vec![90, 24, 35, 67]));
}
