//! 包注释
/*!
    包注释
*/

///文档注释
/**
    文档注释
*/
fn test_comments() {
    //代码注释
    /*
        代码注释
    */
    println!("hello rbe");

    let x = 5 + /* 100 + */ 5;
    println!("x = {x}");
}

fn test_formatted_print() {
    println!("{} days", 10);
    println!("{0}_{1}_{1}_{0}", "hello", "rbe");
    println!("{a} {b} {c}",
            a = "aaa",
            b = "bbb",
            c = "ccc");
    
    #[derive(Debug)]
    struct Structure(i32);
    println!("Structure = {:?}", Structure(3));

    let pi = 3.141592;
    println!("{0} is roughly {pi:.3}", "pi");
}

fn test_debug() {
    #[derive(Debug)]
    struct DebugPrintable(i32);
    
    #[derive(Debug)]
    struct Deep(DebugPrintable);
    
    println!("debug_print = {:?}", DebugPrintable(666));
    println!("{0:?}_{1:?} is {user}'s name",
            "Masker",
            "Dad",
            user = "zq");
    println!("Deep = {:?}", Deep(DebugPrintable(100)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let user = Person {
        name: "makerdad",
        age: 27,
    };
    println!("user: {:?}", user);
    println!("user: {:#?}", user);
}

fn test_display() {
    use std::fmt;
    #[derive(Debug)]
    struct Point2D {
        x: i64,
        y: i64,
    }
    impl fmt::Display for Point2D {
        fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            /*
            write!(f, "({}, {})",
                format!("{:#b}", self.x),
                format!("{:#b}", self.y))
            */
            let x = self.x;
            let y = self.y;
            write!(f, "(")?;
            fmt::Binary::fmt(&x, f)?;
            write!(f, ", ")?;
            fmt::Binary::fmt(&y, f)?;
            write!(f,")")
        }
    }


    let point = Point2D {
        x: 1,
        y: 2,
    };
    println!("Debug: {point:?}");
    println!("Display: {point}");
    println!("Binary: {point:b}");
    
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
    let complex = Complex {
        real: 3.3,
        imag: 7.2,  
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

fn test_list() {
    use std::fmt;
    struct List(Vec<i32>);
    
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (i, v) in vec.iter().enumerate() {
                if i != 0 {
                    write!(f, ", {i}: ")?;
                } else {
                    write!(f, "{i}: ")?;
                }
                write!(f, "{v}")?;
            }
            write!(f, "]")
        }
    }
    
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", List(v));
}

fn test_formatting() {
    use std::fmt::{self, Formatter, Display};

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }
    
    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                    self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    
    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "RGB ({}, {}, {}) ", self.red, self.green, self.blue)?;
            write!(f, "0x{}{}{}",
                    format!("{:02X}", self.red),
                    format!("{:02X}", self.green),
                    format!("{:02X}", self.blue))
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // 在添加了针对 fmt::Display 的实现后，请改用 {} 检验效果。
        println!("{}", *color)
    }
}

fn main() {
    //test_comments();
    //test_formatted_print();
    //test_debug();
    //test_display();
    //test_list();
    test_formatting();
    println!("Hello, world!");
}