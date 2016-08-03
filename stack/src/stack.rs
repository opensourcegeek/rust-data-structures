/**
Stack declaration - structs don't need to be mutable ref as
they inherit mut/immutability from caller
http://stackoverflow.com/questions/29598264/returning-a-struct-containing-mutable-values
*/

pub struct Stack {
    pub data: Vec<i32>
}


/**
Stack methods
*/
impl Stack {
    pub fn push(&mut self, item: i32) -> () {
        let mut d = &mut self.data;
        d.push(item);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let mut d = &mut self.data;
        d.pop()
    }

    pub fn print(&self) -> () {
        for item in &self.data {
            println!("Item {}", item);
        }
    }
}

pub fn new() -> Stack {
    let vec: Vec<i32> = Vec::new();
    let s = Stack {
        data: vec
    };
    s
}
