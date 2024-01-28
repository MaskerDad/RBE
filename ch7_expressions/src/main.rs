fn test() {
    let x = 5u32;
    
    let y = {
        let num1 = x * x; //25
        let num2 = num1 * 100; //250
        num1 + num2
    };

    let z = {
        100 * x;
    };

    println!("x = {:?}", x);
    println!("y = {:?}", y);
    println!("z = {:?}", z);
}

fn main() {
    test();
    println!("Hello, world!");
}