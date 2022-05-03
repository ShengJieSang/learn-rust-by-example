#![allow(dead_code)]

use List::*;

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
struct Unit;
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
struct Pair(i32, f32);

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

#[allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => {
            println!("page loaded")
        }
        WebEvent::PageUnload => {
            println!("page unloaded")
        }
        WebEvent::KeyPress(c) => {
            println!("pressed '{}'.", c)
        }
        WebEvent::Paste(s) => {
            println!("pasted \"{}\".", s)
        }
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y)
        }
    }
}
impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.top_left;
        let Point { x: x2, y: y2 } = self.bottom_right;

        (x2 - x1) * (y2 - y1)
    }
}

fn square(point: Point, length: f32) -> Rectangle {
    let x = point.x + length;
    let y = point.y + length;

    Rectangle {
        top_left: point,
        bottom_right: Point { x, y },
    }
}
fn main() {
    // chapter 3.0 自定义类型
    // chapter 3.1 结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: right_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: right_edge,
        },
        bottom_right,
    };
    println!("the rectangle is {:?}", rectangle);
    println!("the rectangle area is {}", rectangle.rect_area());
    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("square is {:?}", square(point, 2.3));

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;

    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => {
            println!("The rich have lots of money!")
        }
        Poor => {
            println!("The poor have no money...")
        }
    }

    match work {
        Civilian => {
            println!("Civilian work")
        }
        Soldier => {
            println!("Soldier work")
        }
    }

    enum Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify())
}
