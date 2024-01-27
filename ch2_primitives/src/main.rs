#[allow(unused)]
fn test_type() {
    let default_float = 3.0; //f64
    let default_integer = 6; //i32
    let a: f32 = 1.0;
    let b = 10u8;
}

#[allow(unused)]
fn test_literals_operators() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    //println!("1 - 2 = {}", 1u32 - 2); //overflow
}

#[allow(unused)]
fn test_tuples() {
    fn reverse_pair(p: (i32, bool)) -> (bool, i32) {
        let (a, b) = p;
        (b, a)
    }
    
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    
    use std::fmt::{self, Display, Formatter};
    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "({} {})\n({} {})",
                    self.0, self.1, self.2, self.3)
        }
    }
    let m = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", m);

    fn transpose_matrix(m: Matrix) -> Matrix {
        Matrix(
            m.0, m.2,
            m.1, m.3
        )
    }
    let m = transpose_matrix(m);
    println!("Transpose:\n{}", m);
}

#[allow(unused)]
fn test_arrays_slices() {
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    use std::mem;
    let arr1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let arr2: [i32; 10] = [6; 10];
    println!("arr1.last = {}", arr1[arr1.len() - 1]);
    println!("arr1 occupies {} bytes", mem::size_of_val(&arr1));
    analyze_slice(&arr1);
    analyze_slice(&arr1[2..]);
}

fn main() {
    //test_literals_operators();
    //test_tuples();
    test_arrays_slices();
    println!("Hello, world!");
}