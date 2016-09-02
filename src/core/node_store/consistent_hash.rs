
extern crate core;
use std::collections::HashSet;
use core::utils::big::int::BigInt;
use core::utils::sort;


pub struct ConsistentHash{
	// tempnodes: HashSet<BigInt>,
	nodes:  Vec<BigInt>,// [BigInt; 0],
}

//
impl sort::Interface for ConsistentHash{
	fn Len(&self) -> u64{
		self.nodes.len() as u64
	}

	fn Less(&self, i: u64, j: u64) -> bool{
		let mut a: &BigInt = &BigInt::New();
		match self.nodes.get(i as usize).as_mut() {
		    Some(value) => {
		    	println!("got a value: {:?}", value);
		    	a = value;
			},
		    None => println!("an error occurred"),
		}
		let mut b: &BigInt = &BigInt::New();
		match self.nodes.get(j as usize).as_mut() {
		     Some(value) => b = value,
		     None => println!("an error occurred"),
		}

		if a.Cmp(b) > 0i8 {
			return true;
		}
		false
	}

	fn Swap(&mut self, i: u64, j: u64){
		self.nodes.swap(i as usize,j as usize);
	}

}

impl ConsistentHash{
	pub fn new() -> ConsistentHash{
		// let hash: HashSet<BigInt> = HashSet::new();
		ConsistentHash{
			nodes: vec![],
		}
	}
	pub fn add(&mut self, id: BigInt){
		self.nodes.push(id);
		self.nodes.dedup();
		sort::Sort(self);
	}
}

