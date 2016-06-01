
//extern crate std::string;
// extern crate std;
// use std::time::Duration;


use std::collections::HashMap;

pub fn run(){
	println!("hello");
}


pub struct NodeManager{
	SpacingInterval: i32,//超级节点之间查询的间隔时间
	Root:            super::node::Node,
	IsNew:           bool,         //是否是新节点
	Nodes:           HashMap<String,  super::node::Node>, //id符串为键
	consistenHash:   super::consistent_hash::ConsistentHash,
}

impl NodeManager{
	pub fn New() -> NodeManager{
		NodeManager{
			SpacingInterval: 15,
			Root: super::node::Node::New(),
			IsNew: false,
			Nodes: HashMap::new(),
			consistenHash: super::consistent_hash::ConsistentHash::new(),
		}
	}
	pub fn Add(&mut self, node: super::node::Node){
		self.consistenHash.add(node.GetID());
	}
}