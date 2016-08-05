mod stack;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    //println!("Hello, world!");
    let ret = add(1, 2);
    println!("Add: {}", ret);
    
    let mut a = stack::new::<i32>();

    a.push(1);
    a.push(2);

    a.print();

    let poppedItem = a.pop();
    match poppedItem {
        Some(i) => println!("Popped item {}", i),
        _ => println!("No item popped")
    }
    a.print();

    let poppedAgainItem = a.pop();

    match poppedAgainItem {
        Some(i) => println!("Popped again item {}", i),
        _ => println!("No more item popped")
    }
    a.print();
}
