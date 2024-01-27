//! casting/literal/type_inference/alias
#![allow(dead_code)]

fn test_casting() {
    let pid: isize = -1;
    println!("pid(isize): {}", pid);
    println!("pid(usize): {}", pid as usize);
    println!("usize::MAX: {}", usize::MAX);
}

fn test_literal() {
    let x = 1u8;
    let y = 2u32;
    let z = 3usize;
    let i = 1;
    let f = 6.6;
    
    println!("size of x in bytes: {}", std::mem::size_of_val(&x));
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));
}

fn test_inference() {
    let elem = 5u8;
    let mut v = Vec::new();
    v.push(elem);
    println!("{:?}", v);
}

fn test_alias() {
    #[allow(non_camel_case_types)]
    type u64_t = u64;

    type TypeU64 = u64;
    
    let a = 1u8;
    let b = 2u32;
    let c = a as u64_t + b as TypeU64;
    println!("c = {}", c);
}

fn main() {
    test_casting();
    test_literal();
    test_inference();
    test_alias();
    println!("Hello, world!");
}
