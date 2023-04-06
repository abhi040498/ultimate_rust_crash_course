
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

println!("Now using operator function");
operators();
}

fn operators()
{
    // Arithemetic.
    let mut a = 2+3*4; 
    println!("{}", a);
    a = a+1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a,3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise operator
    let c = 1 | 2;  // |OR & AND ^XOR ! NOR
    println!("1|2 {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 ={}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI <4.0; //true
    
    let x = 5;
    let x_is_5 = 5 == 5; //true

}