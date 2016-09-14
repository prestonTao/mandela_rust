
//http://blog.csdn.net/mergerly/article/details/7989281

extern crate core;
use std::collections::HashSet;
use core::utils::big::int::BigInt;
use core::utils::big::parse;
use core::utils::sort;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Kademlia{
	// tempnodes: HashSet<BigInt>,
	find:   BigInt, //需要对比的节点id
	nodes:  Vec<BigInt>,// [BigInt; 0],
}

//从小到大排序
impl sort::Interface for Kademlia{
	fn Len(&self) -> u64{
		self.nodes.len() as u64
	}

	fn Less(&self, i: u64, j: u64) -> bool{
		let mut a = BigInt::new();
		match self.nodes.get(i as usize).as_mut() {
		    Some(value) => {
		    	// println!("got a value: {:?}", &value.Format(16));
		    	a = value.copy();
			},
		    None => println!("an error occurred"),
		}
		let mut b = BigInt::new();
		match self.nodes.get(j as usize).as_mut() {
		     Some(value) => b = value.copy(),
		     None => println!("an error occurred"),
		}

		// let rootBI = parse::ParseInt(self.find.to_string(), super::node_manager::IDbit);
		// let mut c = *a;
		// println!("a {}\nb {}", a.Copy().Format(16), b.Copy().Format(16));
		a.xor(&self.find);
		b.xor(&self.find);

		// println!("a距离 {}\nb距离 {}", a.Copy().Format(10), b.Copy().Format(10));


		if a.cmp(&b) > 0i8 {
			// println!("a 大");
			false
		}else{
			// println!("b 大");
			true
		}
	}

	fn Swap(&mut self, i: u64, j: u64){
		self.nodes.swap(i as usize,j as usize);
	}

}

impl Kademlia{
	pub fn new() -> Kademlia{
		// let hash: HashSet<BigInt> = HashSet::new();
		Kademlia{
			find: BigInt::new(),
			nodes: vec![],
		}
	}
	/*
		添加
	*/
	pub fn add(&mut self, id: BigInt){
		self.nodes.push(id);
		self.nodes.dedup();//去重复
		// sort::Sort(self);
	}

	/*
		获得一个邻近节点，左右距离都相等则取左边
		
	*/
	pub fn get(&mut self, id: BigInt) -> &Vec<BigInt> {

		self.find = id;
		sort::Sort(self);
		&self.nodes
	}

}

