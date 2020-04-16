
// extern crate core;
use std::collections::HashSet;
use super::super::utils::big::int::BigInt;
use super::super::utils::sort;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ConsistentHash{
	// tempnodes: HashSet<BigInt>,
	nodes:  Vec<BigInt>,// [BigInt; 0],
}

//从小到大排序
impl sort::Interface for ConsistentHash{
	fn Len(&self) -> u64{
		self.nodes.len() as u64
	}

	fn Less(&self, i: u64, j: u64) -> bool{
		let mut a: &BigInt = &BigInt::new();
		match self.nodes.get(i as usize).as_mut() {
		    Some(value) => {
		    	// println!("got a value: {:?}", value);
		    	a = value;
			},
		    None => println!("an error occurred"),
		}
		let mut b: &BigInt = &BigInt::new();
		match self.nodes.get(j as usize).as_mut() {
		     Some(value) => b = value,
		     None => println!("an error occurred"),
		}

		if a.cmp(b) > 0i8 {
			return false;
		}
		true
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
	/*
		添加
	*/
	pub fn add(&mut self, id: BigInt){
		self.nodes.push(id);
		self.nodes.dedup();//去重复
		sort::Sort(self);
	}

	/*
		获得一个邻近节点，左右距离都相等则取左边
		
	*/
	pub fn get(&self, id: BigInt) -> Option<BigInt> {
		let length = self.nodes.len();
		println!("length {}", length);
		if length == 0 {
			return None;
		}
		if length == 1{
			match self.nodes.get(0){
				Some(x) => {
					// println!("x: {:?}", x);
					return Some(x.copy());
				},
				None => {
					return None;
				},
			}
		}
		let mut count = 0;
		let mut isFind = false;
		for one in &self.nodes{
			

			match id.cmp(&one){
				0 => {
					return Some(id);
				},
				1 => {
					//比较最后一个和第一个的距离
					if count == length-1 {
						println!("1111111111111111111");
						match self.nodes.get(0){
							Some(x) => {
								let mut left = id.copy();
								left.xor(one);
								let mut right = id.copy();
								right.xor(x);
								match left.cmp(&right){
									0 => {return Some(one.copy());},
									-1 => {return Some(one.copy());},
									1 => {return Some(x.copy());},
									_ => {return None;},
								}
								// return Some(x.Copy());
							},
							None => {
								return None;
							},
						}
					}else{
						//找到位置了，对比当前位置和下一个位置
						println!("22222222222222222");
						match self.nodes.get(count + 1){
							Some(x) => {
								let mut left = id.copy();
								left.xor(one);
								println!("id  {:?}  one  {:?}  result  {:?}", id, one, left);
								let mut right = id.copy();
								right.xor(x);
								println!("left {:?} right {:?}", &left, &right);
								match left.cmp(&right){
									0 => {return Some(one.copy());},
									-1 => {return Some(one.copy());},
									1 => {return Some(x.copy());},
									_ => {return None;},
								}
								// return Some(x.Copy());
							},
							None => {
								return None;
							},
						}
						
					}
				},
				-1 => {
					//比较第一个和最后一个的距离
					if count == 0{
						println!("333333333333333333");
						match self.nodes.get(length - 1){
							Some(x) => {
								let mut left = id.copy();
								left.xor(x);
								let mut right = id.copy();
								right.xor(one);
								match left.cmp(&right){
									0 => {return Some(x.copy());},
									-1 => {return Some(x.copy());},
									1 => {return Some(one.copy());},
									_ => {return None;},
								}
								// return Some(x.Copy());
							},
							None => {
								return None;
							},
						}
					}else{
						//对比下一个
						println!("44444444444444444");
					}
				},
				_ => {
					return None;
				},
			}

			count = count + 1;
		}
		return None;
	}

}

