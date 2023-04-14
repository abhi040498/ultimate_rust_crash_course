
    
    // Stack
    
    fn new_stack(maxsize:usize) -> Vec<u32> {
        let vec: Vec<u32> = Vec::with_capacity(maxsize);
        vec
    }
    
    fn pop(stack: &mut Vec<u32>) -> Option<u32> {
        let poped_val: Option<u32> = stack.pop();
        println!("The popped value is {:?} ", poped_val);
        poped_val
    }
    
    fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
        if stack.len() == maxsize {
            println!("cannot add more");
        } else {
            stack.push(item);
            println!("Stack: {:?} ", stack);
        }
    }
    
    fn size(stack: &Vec<u32>) -> usize {
        stack.len()
    }
    
    fn input() -> u32 {
        let mut n:String = String::new();
        std::io::stdin()
        .read_line(buf: &mut n) Result<usize, Error>
        .expect(msg: "Could not read input");
        
        let n: u32 = n.trim().parse.expect(msg: "invalid input");
        n
    }
    
    fn use_stack() {
        println!("Lets first create stack of our use");
        println!("PLease input the size of the stack");
        let size_stack: u32 = input();
    
        let mut stack: Vec<u32> = new_stack(size_stack as usize);
        
        loop {
        println!("\n\n ********* menu ******** ");
        println!("\n\n ********* menu ******** ");
        println!("1. push  \n2. pop \n3. Display  \n4. size  \n5. Exit");
        println!("\n Enter your choice: ");
        let choice: u32 = input();
        match choice {
            1 => {
                println!("Enter the value of choice: ");
                let item: u32 = input();
                push(&mut stack, item, size_stack as usize);
            }
    
            2 => println!("Enter the element to be popped: {:?}", pop(&mut stack)),
            3 => println!("The elements are: {:?}",stack),
            4 => println!("The size of the stack is: {}",size(&stack)),
            5 => println!("Exiting!"),
            _ => println!("Invalid choice"),
        }
        println!("\n\n ********* Do you wish to continue? ******** \n 1. yes, 2. no");
        let status: u32 = input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    } 
    }