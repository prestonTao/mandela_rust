extern crate mandela_rust;

use mandela_rust::core;

fn main() {

	// bigExample();
	// sortExample();
	nodeStoreExample();

}

fn nodeStoreExample(){
	let mut manager = core::node_store::node_manager::NodeManager::New();
	let mut node = core::node_store::node::Node::New();
	manager.Add(node);

}

