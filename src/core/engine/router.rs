use std::collections::HashMap;
use std::sync::Arc;

pub type Handle = fn();


#[derive(Debug)]
pub struct Router{
    map: HashMap<usize, Handle>,
}

impl Router{
    pub fn new() -> Router{
        Router{map: HashMap::new()}
    }
    pub fn addHandle(&mut self, msgID: usize,handle: Handle){
        self.map.insert(msgID, handle);
    }
}


