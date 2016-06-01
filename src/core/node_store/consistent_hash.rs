
extern crate core;
use std::collections::HashSet;
use core::utils::big::int::BigInt;
use core::utils::sort;


pub struct ConsistentHash{
	nodes: HashSet<BigInt>,
}

impl sort::Interface for ConsistentHash{
	fn Len(&self) -> usize{
		0
	}

	fn Less(&self, i: usize, j: usize) -> bool{
		true
	}

	fn Swap(&self, i: usize, j: usize){

	}

}

impl ConsistentHash{
	pub fn new() -> ConsistentHash{
		let hash: HashSet<BigInt> = HashSet::new();
		ConsistentHash{
			nodes: hash,
		}
	}
	pub fn add(&mut self, id: &BigInt){
		sort::Sort(self);
	}
}
