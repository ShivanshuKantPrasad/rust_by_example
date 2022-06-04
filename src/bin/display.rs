use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("{}", Structure(23));
    println!("{:?}", Structure(23));
    println!("{:#?}", Structure(23));

    println!("{}", MinMax(23, 32));
    println!("{:?}", MinMax(23, 32));
    println!("{:#?}", MinMax(23, 32));

    println!("{}", Point2D { x: 2.3, y: 3.2 });
    println!("{:?}", Point2D { x: 2.3, y: 3.2 });
    println!("{:#?}", Point2D { x: 2.3, y: 3.2 });

    println!(
        "{}",
        Complex {
            real: 2.3,
            imag: 3.2
        }
    );
    println!(
        "{:?}",
        Complex {
            real: 2.3,
            imag: 3.2
        }
    );
    println!(
        "{:#?}",
        Complex {
            real: 2.3,
            imag: 3.2
        }
    );
}
