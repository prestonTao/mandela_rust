extern crate mandela_rust;

use mandela_rust::core;



#[test]
fn bigExample(){
	
	let idinforoot = core::node_store::node::IDInfo::new("tao", "tao@126.com", "mandela", "34ed23f3a4609c");
	println!("id {}", &idinforoot.ID.Format(16));
	let root = core::node_store::node::Node::new(idinforoot, true, "127.0.0.1", 9981, 19981);
	let mut manager = core::node_store::node_manager::NodeManager::New(root);


	let idinfo = core::node_store::node::IDInfo::new("tao1", "tao@126.com", "baidu", "34ed23f3a4609c");
	println!("id1 {}", &idinfo.ID.Format(16));
	let node = core::node_store::node::Node::new(idinfo, true, "127.0.0.1", 9981, 19981);
	manager.AddNode(node);


	let idinfo = core::node_store::node::IDInfo::new("tao2", "tao@126.com", "xunlei", "34ed23f3a4609c");
	println!("id2 {}", &idinfo.ID.Format(16));
	let node = core::node_store::node::Node::new(idinfo, true, "127.0.0.1", 9981, 19981);
	manager.AddNode(node);



	let idinfo = core::node_store::node::IDInfo::new("tao3", "tao@126.com", "duowan", "34ed23f3a4609c");
	println!("id3 {}", &idinfo.ID.Format(16));
	let node = core::node_store::node::Node::new(idinfo, true, "127.0.0.1", 9981, 19981);
	manager.AddNode(node);



	let idinfo = core::node_store::node::IDInfo::new("tao4", "tao@126.com", "github", "34ed23f3a4609c");
	println!("id4 {}", &idinfo.ID.Format(16));
	let node = core::node_store::node::Node::new(idinfo, true, "127.0.0.1", 9981, 19981);
	manager.AddNode(node);


	let idinfo = core::node_store::node::IDInfo::new("tao4", "tao@126.com", "github", "34ed23f3a4609c");
	let arcNode = manager.Get(&idinfo.ID.Format(16), true, "1e5c1ca4d0cd0fab3e9febb51b450f0530ff15812bf0c48d41efe377d3dc43180fa9600ee68b2f746a81d0990e38ac2c1cee70929f0dcb67fdae6659d77b9ff8", 10);
	for one in arcNode{
		let temp = one.clone();
		let temp2 = temp.read().unwrap();
		let id = temp2.getID();

		let mut idcopy = id.Copy();
		let rootid = core::node_store::node::IDInfo::new("tao4", "tao@126.com", "github", "34ed23f3a4609c").ID;
		idcopy.Xor(&rootid);
		println!("找到 {}\n距离 {}", &id.Format(16), idcopy.Format(10));
	}
}

