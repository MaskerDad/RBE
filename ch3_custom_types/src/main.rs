//! struct/enum/constant
#![allow(dead_code)]

// Tuple/Unit/C
fn test_struct() {
    //claim
    #[derive(Debug)]
    struct Unit;
    
    #[derive(Debug)]
    struct Pair(i32, f32);
    
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f32,
        y: f32,
    }
    
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    //init
    let uint = Unit;
    let pair = Pair(1, 2.2);
    let point1 = Point {
        x: 6.0,
        y: 8.0,
    };
    let point2 = Point {
        x: 10.0,
        y: 20.0,
    };
    let rectangle = Rectangle {
        top_left: point1,
        bottom_right: point2,
    };
    println!("Unit:\n{:?}", uint);
    println!("Tuple:\n{:?}", pair);
    println!("Struct_of_Struct:\n{:?}", rectangle);

    //destruct
    let point3 = Point {
        x: 1.0,
        y: 2.0,
    };
    let Pair(x, y) = pair;
    let Point{x: x_d, y: y_d} = point3;
    println!("pair.0 = {}, pair.1 = {}", x, y);
    println!("point3.x = {}, point3.y = {}", x_d, y_d);

    println!("/* try: area */");
    fn rectangle_area(rectangle: &Rectangle) -> f32 {
        let Rectangle {
            top_left: Point {
                x: top_left_x,
                y: top_left_y,
            },
            bottom_right: Point {
                x: bottom_right_x,
                y: bottom_right_y,
            }
        } = rectangle;
        let a = bottom_right_x - top_left_x;
        let b = top_left_y - bottom_right_y;
        a * b
    }
    let rectangle = Rectangle {
        top_left: Point{x: 1.0, y: 8.0},
        bottom_right: Point{x: 8.0, y: 2.0},
    };
    println!("area = {}", rectangle_area(&rectangle)); //42

    println!("/* try: square */");
    fn square(p: Point, size: f32) -> Rectangle {
        Rectangle {
            top_left: p,
            bottom_right: Point {
                x: p.x + size,
                y: p.y - size,
            },
        }
    }
    let p = Point{x: 1.0, y: 8.0};
    let size = 2.0;
    println!("square:\n{:?}", square(p, size));
}

fn test_enum() {
    enum WebEvent {
        PageLoad,
        KeyPress(char),
        Paste(String),
        Click{x: i64, y: i64},
    }

    impl WebEvent {
        fn inspect(event: WebEvent) {
            match event {
                Self::PageLoad => println!("PageLoad"),
                Self::KeyPress(c) => println!("KeyPress: {c}"),
                Self::Paste(s) => println!("Paste: {s}"),
                Self::Click { x, y } => {
                    println!("Click x,y = ({x}, {y})");  
                },
            }
        }
    }

    let load = WebEvent::PageLoad;
    let pressed = WebEvent::KeyPress('a');
    let pasted = WebEvent::Paste("rust".to_owned());
    let clicked = WebEvent::Click { x: 6, y: 10 };
    WebEvent::inspect(load);
    WebEvent::inspect(pressed);
    WebEvent::inspect(pasted);
    WebEvent::inspect(clicked);

    /* C-Style Enum */
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

    println!("zero: {}", Number::One as i32);
    println!("Red: {:#06x}", Color::Red as i32);
}

fn test_enum_list() {
    use List::*;
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> Self {
            Nil
        }

        fn push_front(self, val: u32) -> Self {
            Cons(val, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref child) => child.len() + 1,
                Nil => 0,
            }
        }

        fn to_string(&self) -> String {
            match *self {
                Cons(val, ref child) => {
                    format!("{} {}", val, child.to_string())
                },
                Nil => format!("Nil"),
            }
        }
    }

    let mut list = List::new();
    for i in (0..3).rev() {
        list = list.push_front(i);
    }
    
    println!("list len: {}", list.len()); //3
    println!("{}", list.to_string()); //0 1 2 Nil
}

static LANGUAGE: &str = "Rust";
static mut NUM_1: i32 = 16;
const NUM_2: f64 = 8.0;

fn test_constant() {
    println!("{}", LANGUAGE);
    unsafe {
        NUM_1 += 1;
        println!("{}", NUM_1);
    }
    println!("{}", NUM_2);
}

fn main() {
    //test_struct();
    //test_enum();
    //test_enum_list();
    test_constant();
    println!("Hello, world!");
}