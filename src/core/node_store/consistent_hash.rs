
extern crate core;
use std::collections::HashSet;
use core::utils::big::int::BigInt;
use core::utils::sort;


pub struct ConsistentHash{
	nodes: HashSet<BigInt>,
}

//
impl sort::Interface for ConsistentHash{
	fn Len(&self) -> u64{
		self.nodes.len() as u64
	}

	fn Less(&self, i: u64, j: u64) -> bool{
		// for (index, one) in s.drain().by_ref().take(2).enumerate(){
		// 	println!("{} {}", index, one);
		// }
		let mut index_i = BigInt::New();
		let mut index_j = BigInt::New();
		for (index, one) in self.nodes.enumerate() {
			if index == i{
				index_i = one;
				continue;
			}
			if index_j == j{
				index_j = one;
				continue;
			}
		}
		if index_i.Cmp(index_j) > 0 {
			return true;
		}
		false
	}

	fn Swap(&mut self, i: u64, j: u64){
		let hash: HashSet<BigInt> = HashSet::new();
		if i > j{
			let mut index_i = BigInt::New();
			let mut index_j = BigInt::New();
			for (index, one) in self.nodes.enumerate() {
				if index == i{
					index_i = one;
					continue;
				}
				if index_j == j{
					index_j = one;
					continue;
				}
			}
			for (index, one) in (self.nodes).enumerate() {
			    if index == i{
			    	match self.nodes.get(i){
			    		Some(x) => hash.insert(x),
			    		None => {},
			    	}
			    	continue;
			    }
			    if index == j{
			    	match self.nodes.get(j){
			    		Some(x) => {
			    			hash.insert(*x);
			    		},
			    		None => {
			    			continue;
			    		},
			    	}
			    	continue;
			    }
			    hash.insert(one);
			}
		}else{

		}
		
		self.nodes = hash;
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
		self.nodes.insert(*id);
		sort::Sort(self);
	}
}

