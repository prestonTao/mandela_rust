
//extern crate std::string;
// extern crate std;
// use std::time::Duration;


use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
// use core::ops::Deref;


pub const IDbit: u8 = 10; 



pub struct NodeManager{
	// SpacingInterval: i32,//超级节点之间查询的间隔时间
	Root:            super::node::Node, //本节点信息
	// IsNew:           bool,         //是否是新节点
	nodes:           RwLock<HashMap<String, Arc<RwLock<super::node::Node>>>>,//超级节点，id符串为键
	Proxys:          RwLock<HashMap<String, Arc<RwLock<super::node::Node>>>>,//被代理的节点，id字符串为键
	// consistenHash:   super::consistent_hash::ConsistentHash,
}

impl NodeManager{
	pub fn New() -> NodeManager{
		// let rwMap = HashMap::new();
		NodeManager{
			// SpacingInterval: 15,
			Root: super::node::Node::New(),
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
		self.Proxys.write().unwrap().insert(arcNodeTwo.read().unwrap().GetID().Format(IDbit).to_string(), arcNode);
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
		self.nodes.write().unwrap().insert(arcNodeTwo.read().unwrap().GetID().Format(IDbit).to_string(), arcNode);
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
		let mut kd = super::kademlia::Kademlia::new();
		if includeSelf{
			kd.add(self.Root.GetID().Copy());
		}
		for (key, value) in self.nodes.read().unwrap().iter() {
			let a = value.clone();
			let b = a.read().unwrap().GetID().Copy();
			if !includeSelf {
				if b.Copy().Cmp(&self.Root.GetID()) == 0{
					continue
				}
			}
			kd.add(b);
		}

		let nodeids = kd.get(nodeId);
		let mut result: Vec<Arc<RwLock<super::node::Node>>> = vec![];
		result.reserve(num as usize);
		for i in 0..num {
			match nodeids.get(i as usize){
				Some(x) => {
					let nodes = self.nodes.read().unwrap();
					match nodes.get(&x.Format(IDbit)){
						None => {
							//TODO 找不到后，返回结果长度变少，应该补上
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
}	