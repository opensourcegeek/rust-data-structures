use std::fmt::Display;

/**
Stack declaration - structs don't need to be mutable ref as
they inherit mut/immutability from caller
http://stackoverflow.com/questions/29598264/returning-a-struct-containing-mutable-values
*/

pub struct Stack<T> {
    pub data: Vec<T>
}


/**
Stack methods
*/
impl<T: Display> Stack<T> {
    pub fn push(&mut self, item: T) -> () {
        let mut d = &mut self.data;
        d.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut d = &mut self.data;
        d.pop()
    }

    pub fn print(&self) -> () {
        for item in &self.data {
            println!("Item {}", item);
        }
    }
}

pub fn new<T>() -> Stack<T> {
    let vec: Vec<T> = Vec::new();
    let s = Stack {
        data: vec
    };
    s
}
