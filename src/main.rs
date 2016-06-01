extern crate mandela;

use mandela::core;

fn main() {

	// bigExample();
	nodeStoreExample();

}

fn nodeStoreExample(){
	let mut manager = core::node_store::node_manager::NodeManager::New();
}


fn bigExample(){


	let mut src = core::utils::big::parse::ParseInt("011011101111".to_string(), 2);
	let mut dst = core::utils::big::parse::ParseInt("01010".to_string(), 2);

	//解析
	//正数
	src = core::utils::big::parse::ParseInt("011011101111".to_string(), 2);
	src = core::utils::big::parse::ParseInt("1775".to_string(), 10);
	src = core::utils::big::int::BigInt::NewInt(1775);
	//负数
	src = core::utils::big::parse::ParseInt("111011101111".to_string(), 2);
	src = core::utils::big::parse::ParseInt("-1775".to_string(), 10);
	src = core::utils::big::int::BigInt::NewInt(-1775);
	println!("格式化 {}", src.Format(10));
	println!("格式化 {}", src.Format(2));


	
	src = core::utils::big::parse::ParseInt("0111111111".to_string(), 2);
	src.Lsh(9);
	println!("左移 {}", src.Format(2));


	src.Rsh(1);
	println!("右移 {}", src.Format(2));


	src.Not();
	println!("取反 {}", src.Format(2));

	let mut dstBigInt = core::utils::big::parse::ParseInt("011".to_string(), 2);
	src.Xor(&dstBigInt);
	println!("异或 {}", src.Format(2));


	src = core::utils::big::parse::ParseInt("-10".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("-11".to_string(), 10);
	println!("对比 {}", src.Cmp(&dst));

	src = core::utils::big::parse::ParseInt("-10".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("-1".to_string(), 10);
	src.Add(&dst);
	println!("加法 {}", src.Format(10));

	src = core::utils::big::parse::ParseInt("18".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("10".to_string(), 10);
	src.Sub(&dst);
	println!("减法 {}", src.Format(10));


	src = core::utils::big::int::BigInt::NewInt(9998);
	dst = core::utils::big::int::BigInt::NewInt(9907);
	src.Mul(&dst);
	println!("乘法 {}", src.Format(10));


	src = core::utils::big::parse::ParseInt("9997".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("10".to_string(), 10);
	src.Div(&dst);
	println!("除法 {}", src.Format(10));


	src = core::utils::big::int::BigInt::NewInt(99999999999999997);
	dst = core::utils::big::int::BigInt::NewInt(10);
	src.Mod(&dst);
	println!("取模 {}", src.Format(10));


	src = core::utils::big::parse::ParseInt("-1775".to_string(), 10);
	dst = core::utils::big::int::BigInt::NewInt(10);
	src.Mod(&dst);
	println!("取模 {}", src.Format(10));
}