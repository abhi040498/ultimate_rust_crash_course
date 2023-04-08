
use std::collections::HashMap;

// Structs
struct Point 
{
    x: f64,
    y: f64,
}
struct Line 
{
    start: Point,
    end: Point,
}
fn structures()
{
    let p = Point{x: 3.0, y: 4.0};
    println!("Point P is at ({}, {})", p.x, p.y);
    let p2 = Point{x: 5.0, y: 6.0};
    let my_line = Line{start:p, end:p2};
    print!("Line is from ({}, {}) to ({}, {})", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y );
}

// Working with enumerations enum
enum Color
{
    Red,
    Green, 
    Blue,
    RGBColor(u8,u8,u8),     //This is a tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, //A sturct inside enum.
}

pub fn enums()
{
    // let matchColor:Color = Color::Red;
    let matchColor:Color = Color::CmykColor { cyan: 0, magenta: 128, yellow: 0, black: 255 };
    match matchColor
    {
        Color::Red => println!("Printing:- R"),
        Color::Blue => println!("Printing:- B"),
        Color::Green => println!("Printing:- G"),
        // Color::RGBColor(0, 0, 0) => println!("Printing:- Black"),
        // Color::RGBColor(0, 0, 0)
            Color::CmykColor{ cyan:_, magenta:_, yellow:_, black: 255 } => println!("Printing:- Black"),
        Color::RGBColor(r, g, b) => println!("Printing RGB Values:- {}, {}, {}", r, g, b),
        _=> ()
    } 
    touples_define();
}
// Touple is collections of several values.
fn sum_and_product(x:i32, y:i32) -> (i32, i32, i32)
{
    (x+y, x*y, x/y)
}
fn touples_define()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sum and product is {:?}", sp);
}

fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    a.push(44);

    println!("a = {:?}", a);

    // type = unsigned to be used. usize or isize will match the size of machine.
    let idx:usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    // scenario- if we want to get out or range index element then rust will throw error.
    // we use get function. get 
    // get function, instead of just crashing the program completely,
    // it returns an option type.
    // in option we can match against it.
    match a.get(6)
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    // to view all elements of vectors.
    println!("Element of a is {:?}", a);

}

fn ds_hashmap()
{
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);
}