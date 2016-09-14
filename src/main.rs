extern crate mandela_rust;

use mandela_rust::core;

fn main() {


	example();

}


fn example(){
	let idinforoot = core::node_store::node::IDInfo::new("tao", "tao@126.com", "mandela", "34ed23f3a4609c");
	println!("id {}", &idinforoot.ID.format(2));
	let root = core::node_store::node::Node::new(idinforoot, true, "127.0.0.1", 9981, 19981);
	let mut manager = core::node_store::node_manager::NodeManager::New(root);
	manager.getNetworkNum();


	println!("---------------------------------\n");

}



