
//extern crate std::string;
// extern crate std;
// use std::time::Duration;
extern crate core;

use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
// use core::ops::Deref;
use core::utils::big::parse;
use core::utils::big::int::BigInt;


pub const IDbit: u8 = 16; 



pub struct NodeManager{
	// SpacingInterval: i32,//超级节点之间查询的间隔时间
	Root:            Arc<RwLock<super::node::Node>>, //本节点信息
	// IsNew:           bool,         //是否是新节点
	nodes:           RwLock<HashMap<String, Arc<RwLock<super::node::Node>>>>,//超级节点，id符串为键
	Proxys:          RwLock<HashMap<String, Arc<RwLock<super::node::Node>>>>,//被代理的节点，id字符串为键
	// consistenHash:   super::consistent_hash::ConsistentHash,
}

impl NodeManager{
	pub fn New(root: super::node::Node) -> NodeManager{
		// let rwMap = HashMap::new();
		let rwNode = RwLock::new(root);
		let arcNode = Arc::new(rwNode);
		NodeManager{
			// SpacingInterval: 15,
			Root: arcNode,
			// IsNew: false,
			nodes: RwLock::new(HashMap::new()),
			Proxys: RwLock::new(HashMap::new()),
			// consistenHash: super::consistent_hash::ConsistentHash::new(),
		}
	}
	
	/*
		添加一个被代理的节点
	*/
	pub fn AddProxyNode(&mut self, node: super::node::Node){
		let rwNode = RwLock::new(node);
		let arcNode = Arc::new(rwNode);
		let arcNodeTwo = arcNode.clone();
		self.Proxys.write().unwrap().insert(arcNodeTwo.read().unwrap().getID().format(IDbit).to_string(), arcNode);
	}

	/*
		获得一个代理节点
	*/
	pub fn GetProxyNode(&self, id: &str) -> Option<Arc<RwLock<super::node::Node>>>{
		match self.Proxys.read().unwrap().get(id){
			Some(x) => {
				Some(x.clone())
			},
			None => {
				None
			},
		}
	}

	/*
		删除一个被代理的节点
	*/
	pub fn DelProxyNode(&self, id: &str){
		self.Proxys.write().unwrap().remove(id);
	}

	/*
		添加一个超级节点
	*/
	pub fn AddNode(&mut self, node: super::node::Node){
		let rwNode = RwLock::new(node);
		let arcNode = Arc::new(rwNode);
		let arcNodeTwo = arcNode.clone();
		self.nodes.write().unwrap().insert(arcNodeTwo.read().unwrap().getID().format(IDbit).to_string(), arcNode);
	}

	/*
		删除一个超级节点
	*/
	pub fn DelNode(&self, id: &str){
		self.nodes.write().unwrap().remove(id);
	}

	/*
		根据节点id得到一个距离最短节点的信息，不包括代理节点
		@nodeId         要查找的节点
		@includeSelf    是否包括自己
		@outId          排除一个节点
		@return         查找到的节点id，可能为空
		@return         获得数量
	*/
	pub fn Get(&self, nodeId: &str, includeSelf: bool, outId: &str, num: u32) -> Vec<Arc<RwLock<super::node::Node>>> {
		let node = parse::ParseInt(nodeId.to_string(), IDbit);
		let mut kd = super::kademlia::Kademlia::new();
		if includeSelf{
			let temp = self.Root.clone();
			kd.add(temp.read().unwrap().getID().copy());
		}
		for (key, value) in self.nodes.read().unwrap().iter() {
			let a = value.clone();
			let b = a.read().unwrap().getID().copy();
			if !includeSelf {
				let temp = self.Root.clone();
				if b.copy().cmp(&temp.read().unwrap().getID()) == 0{
					continue
				}
			}
			if &b.format(IDbit) == outId{
				continue
			}
			kd.add(b);
		}

		let nodeids = kd.get(node);
		// println!("{:?}", nodeids);
		let mut result: Vec<Arc<RwLock<super::node::Node>>> = vec![];
		result.reserve(num as usize);
		for i in 0..num {
			match nodeids.get(i as usize){
				Some(x) => {
					let nodes = self.nodes.read().unwrap();
					// println!("hashmap长度 {}", nodes.len());
					match nodes.get(&x.format(IDbit)){
						None => {
							//可能是root节点
							let temp = self.Root.clone();
							if temp.read().unwrap().getID().format(16) == x.format(16){
								result.push(self.Root.clone());
								continue;
							}
							//TODO 找不到后，返回结果长度变少，应该补上
							// println!("找不到 {}", i);
							continue;
						},
						Some(x) => {
							result.push(x.clone());
						},
					}
				},
				None => {break},
			}
		}
		result
	}

	/*
		得到每个节点网络的网络号，不包括本节点
	*/
	pub fn getNetworkNum(&self) -> Vec<BigInt>{
		let mut nums: Vec<BigInt> = Vec::new();
		let temp = self.Root.clone();
		let temp = temp.read().unwrap();
		for i in 1..513 {
			let mut src = temp.getID().copy();
			let mut temp = src.copy();
			let mut dst = BigInt::newInt(1);
			dst.lsh(i-1);
			temp.and(&dst);
			src.rsh(i);
			src.lsh(i);
			if temp.cmp(&BigInt::newInt(0)) == 0{
				let mut temp = BigInt::newInt(1);
				temp.lsh(i-1);
				src.or(&temp);
			}
			nums.push(src);
		}
		nums
	}
}	