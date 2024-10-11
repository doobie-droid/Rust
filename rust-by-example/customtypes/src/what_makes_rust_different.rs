// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // You can create a new structure by destructuring a previous point
    let _new_point = Point { x: 2.3, ..point };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate and destructure  a tuple struct
    let pair = Pair(1, 0.1);
    let Pair(_integer, _decimal) = pair;

    // Instantiate and Destructure a normal struct
    let another_point: Point = Point { ..point };
    let Point {
        x: left_edge,
        y: top_edge,
    } = another_point;

    println!("The left edge is  {}", left_edge);
    println!("The top edge is  {}", top_edge);
}
