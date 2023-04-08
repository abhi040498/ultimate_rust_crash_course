use std::mem;
// use rand::Rng;
use std::io::stdin;
struct Point 
{
    x: f64,
    y: f64,
}
enum State {
    Locked,
    Failed,
    Unlocked
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // Working with If-Else
    let temp = 25;
    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Today is {} ", day);

    // We can have if-else in print statement also.

    // Working with while loop
    fn while_loop()
    {
        let mut x = 1;
        while x < 1000
        {
            x *= 2;
            if x == 64 {continue;}
            println!("x = {} ", x);
        }
    }
    // calling while loop function. 
    while_loop();

    // Working with for loop
    fn for_loop()
    {
        for x in 1..10
        {
            if x == 3 {continue; }
            if x ==8 {break; }

            println!("x = {} ", x);
        }

        // getting the position (indexing)  //we can use any var name 1st is index, 2nd is value
        for (position,value) in (30..41).enumerate()
        {
            println!("{} : {}", position, value);
        }
    }
    // calling for loop function 
    for_loop();

    // match statement.
    let country_code = 44;
    
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        91 => "India",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };
    println!("The country with code {} is {} ", country_code, country);

}

// We can call this function whereever we want.
fn combination_lock()
{
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop{
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }
                if entry ==code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}