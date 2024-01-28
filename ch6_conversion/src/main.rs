//! {From/Into}, {TryFrom/TryInto}, {ToString/FromStr}
#![allow(dead_code)]

fn test_from_into() {
    //use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Self {
                value: item,
            }
        }
    }

    let value_i32 = 6i32;
    let number_from_i32 = Number::from(value_i32);
    let number_i32_into: Number = value_i32.into();
    println!("number_from_i32: {:?}", number_from_i32);
    println!("number_i32_into: {:?}", number_i32_into);
}

//return Result<V, E>
fn test_tryfrom_tryinto() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        
        fn try_from(item: i32) -> Result<Self, Self::Error> {
            if item % 2 == 0 {
                Ok(EvenNumber(item))
            } else {
                Err(())
            }
        }
    }

    //try_from
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(7), Err(()));
    //try_into
    let res: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(res, Ok(EvenNumber(8)));
    let res: Result<EvenNumber, ()> = 7i32.try_into();
    assert_eq!(res, Err(()));

    println!("test passed");
}

//  std::fmt::Display <=> ToString
fn test_tostring_fromstr() {
    use std::fmt::{self, Display, Formatter};

    struct Point {
        x: f32,
        y: f32,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "point_({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 1.2, y: 6.6 };
    println!("println: {}", p);
    println!("to_string: {}", p.to_string());

    let num1: i32 = "100".parse().unwrap();
    let num2 = "50".parse::<i32>().unwrap();
    println!("num1 + num2 = {}", num1 + num2);    
}

fn main() {
    //test_from_into();
    //test_tryfrom_tryinto();
    test_tostring_fromstr();
    println!("Hello, world!");
}