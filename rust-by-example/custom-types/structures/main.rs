
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rectangle;
    ( bottom_right.y - top_left.y ) * ( top_left.x - bottom_right.y )
}

fn square(point: Point, edge: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle {
        top_left: point,
        bottom_right: Point { x: x - edge, y: y - edge },
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);
    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);


    let bottom_right = Point { x: 5.2, y: 5.0 };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);


    let Point { x: left_edge, y: top_edge } = point;

    println!("left_edge: {}, top_edge: {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair( integer, decimal ) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rect_area: {}", rect_area(_rectangle));

    println!("square: {:?}", square(Point { x: 10.0, y: 10.0 }, 5.0));
}
