#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32{
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 }
    } = rect;

    let h = y2 - y1;
    let w = x2 - x1;
    (h*w).abs()
}

fn square(point: Point, length: f32) -> Rectangle{
    Rectangle{
        bottom_right: Point {
            x: point.x + length,
            y: point.y + length
        },
        top_left: point,
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 20.5 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructuring the point
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("The area of the rectangle is {}", rect_area(&_rectangle));

}