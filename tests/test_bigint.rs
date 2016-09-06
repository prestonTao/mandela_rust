extern crate mandela_rust;

use mandela_rust::core;

// #[test]
// fn it_works() {
//     assert_eq!(4, adder::add_two(2));
// }

#[test]
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


#[test]
fn sortExample(){
	let mut m1 = sort{num: vec![2,5,3, 9, 24, 5, 7, 1, 100]};
	core::utils::sort::Sort(&mut m1);
	println!("排序过后的数组： {:?}", m1);
}

#[derive(Debug)]
struct sort{
	num: Vec<u8>,
}

impl core::utils::sort::Interface for sort{
	fn Len(&self) -> u64{
		self.num.len() as u64
	}                    // Len is the number of elements in the collection.
	fn Less(&self, i: u64, j: u64) -> bool{
		// println!("{}", self.num[i] > self.num[j]);
		//从大到小排序
		return self.num[i as usize] > self.num[j as usize];
	}  // index i should sort before the element with index j.
	fn Swap(&mut self, i: u64, j: u64){
		let temp = self.num[i as usize];
		self.num[i as usize] = self.num[j as usize];
		self.num[j as usize] = temp;
	}          // Swap swaps the elements with indexes i and j.
}

