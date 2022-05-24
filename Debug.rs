#[derive(Debug)]
struct Empty;

#[derive(Debug)]
struct Structure(String);

#[derive(Debug)]
struct Deep(Structure);

fn main(){
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure("3".to_string()));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure("7".to_string())));

    let o = Empty;

    println!("Now {:?} will print!", o);
}
