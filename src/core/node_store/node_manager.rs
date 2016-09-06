
//extern crate std::string;
// extern crate std;
// use std::time::Duration;


use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::Arc;

pub fn run(){
	println!("hello");
}


pub struct NodeManager{
	SpacingInterval: i32,//超级节点之间查询的间隔时间
	Root:            super::node::Node,
	IsNew:           bool,         //是否是新节点
	Nodes:           RwLock<HashMap<String, Arc<RwLock<super::node::Node>>>>,//id符串为键
	consistenHash:   super::consistent_hash::ConsistentHash,
}

impl NodeManager{
	pub fn New() -> NodeManager{
		// let rwMap = HashMap::new();
		NodeManager{
			SpacingInterval: 15,
			Root: super::node::Node::New(),
			IsNew: false,
			Nodes: RwLock::new(HashMap::new()),
			consistenHash: super::consistent_hash::ConsistentHash::new(),
		}
	}
	pub fn Add(&mut self, node: super::node::Node){
		// let key = node.GetID().Format(64).to_string();
		// let tempNode = &node;
		// let cellNode = RefCell::new(*node);
		let rwNode = RwLock::new(node);
		let arcNode = Arc::new(rwNode);
		let arcNodeTwo = arcNode.clone();
		self.Nodes.write().unwrap().insert(arcNodeTwo.read().unwrap().GetID().Format(64).to_string(), arcNode.clone());
			// let mut rwMap = self.Nodes.write().unwrap();
			// match rwMap.get(&key){
			// 	Some(x) => {},
			// 	None => {
			// 		println!("333333333333333333333333");
			// 		rwMap.insert(key, arcNode);
			// 		// let cellNode = x.deref().write().unwrap();
			// 		// cellNode.borrow_mut().name = "hongfeinihao".to_string();
			// 		println!("4444444444444444444444");
			// 	},
			// }
		


		// {
		// 	let rwMap = self.Nodes.write().unwrap();
		// 	match rwMap.get(&(node.GetID().Format(64))){
		// 		Some(x) => {},
		// 		None => {
		// 			rwMap.insert(key, arcNode);
		// 			// let cellNode = x.deref().write().unwrap();
		// 			// cellNode.borrow_mut().name = "hongfeinihao".to_string();
		// 		},
		// 	}
		// }
		// let temp = &node;
		// self.Nodes.insert(node.GetID().Format(64), node);
		let arcNodeTree = arcNode.clone();

		let bigint = arcNodeTree.read().unwrap();
		self.consistenHash.add(bigint.GetID().Copy());
		println!("finish");
	}
}