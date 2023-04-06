
use std::mem;
fn main() 
{
    // println!("Hello, Rust");
    let a: u8 =123;  //u =unsigned, 0-255;
    println!("a = {}", a);  //immutable

    // u = unsigned, 0 to 2^n-1
    let mut b: i8 = 0;   // -128 to 127
    println!("b = {} before", b);
    b= 42;
    println!("b = {} after", b);

    let mut c= 123456789;   //i32 = 32 bites = 4 bytes
    println!("c = {}, takes up {} bytes",c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    // u8, u18, u32,... i8, i16, i32...

    // usize isize
    let z: isize = 123;
    let size_of_z:usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}- bits OS", z, size_of_z, size_of_z*8);
    
    let d: char = 'x';
    println!("{} is char, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f64 IEEE754 signed!
    let e = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));
    
    let g: bool = false;  //true
    println!("{}, size = {} bytes", e, mem::size_of_val(&g));



}