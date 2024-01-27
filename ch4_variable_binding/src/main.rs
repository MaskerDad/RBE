#![allow(dead_code)]
fn test_variable_binding() {
    let a: isize = -1;
    println!("a = {}", a as usize);
    println!("usize::MAX = {}", usize::MAX);

    let mut mutable_var = 666u32;
    {
        let mutable_var = mutable_var;
        println!("inner var: {}", mutable_var);
    }
    mutable_var += 666;
    println!("outer var: {}", mutable_var);
}

fn main() {
    test_variable_binding();
    println!("Hello, world!");
}
