extern crate mandela_rust;

use mandela_rust::core;

fn main() {

	// bigExample();
	// sortExample();
	// nodeStoreExample();

	example();

}


fn example(){



	let mut manager = core::node_store::node_manager::NodeManager::New();
	let mut node = core::node_store::node::Node::New();
	manager.AddNode(node);

	let mut ch = core::node_store::kademlia::Kademlia::new();
	let int1 = core::utils::big::int::BigInt::NewInt(6);
	ch.add(int1);
	let int1 = core::utils::big::int::BigInt::NewInt(1);
	ch.add(int1);
	let int1 = core::utils::big::int::BigInt::NewInt(8);
	ch.add(int1);

	println!("查找目标 {:?}", ch);

	let findID = core::utils::big::int::BigInt::NewInt(4);
	// let id = findID.Format(10);
	let result = ch.get("4");
	println!("找到结果 {:?}", result);
}



