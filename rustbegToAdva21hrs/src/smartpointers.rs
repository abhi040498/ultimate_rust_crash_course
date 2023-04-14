fn pointers() {
    let single_value = Box::new(0.625);
    let x: f64 = 0.625;
    println!("Are the values being equal {}", x == *single_value);

    let mut stack_var: i32 = 4;
    let stack_ref: & i32 = &stack_var;

    let heap_var: Box<i32> = Box::new(stack_var);
    stack_var = 5;
    println!("The values of stack_var = {} and heap_vare = {}", stack_var, heap_var);

    let point: Box<(i32, i32)> = Box::new((100, 125));

}