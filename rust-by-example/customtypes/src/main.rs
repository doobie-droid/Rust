// An attribute to hide warnings for unused code.
// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
// Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f64 {
        let length = self.top_left.x - self.bottom_right.x;
        let width = self.top_left.y - self.bottom_right.y;
        let length = length.abs() as f64;
        let width = width.abs() as f64;
        return length * width;
    }
}

fn square(point: Point, side: f32) -> Rectangle {
    let top_left = point;
    let bottom_right = Point{ x: top_left.x - side, y: top_left.y - side};
    return Rectangle{top_left, bottom_right};
}
fn main() {
    let top_left: Point = Point { x: 10.3, y: 0.4 };
    let bottom_right: Point = Point { x: 5.2, y: 0.2 };
    let rectangle: Rectangle = Rectangle {top_left, bottom_right };
    println!("The area of the rectangle {:#?} is {1:.2} units square", rectangle, rectangle.rect_area());
    let another_top_left: Point = Point {x: 6.0, y: 8.0};
    let square = square(another_top_left, 4f32);
    println!("The area of the rectangle {:#?} is {1:.2} units square", square, square.rect_area());

}
