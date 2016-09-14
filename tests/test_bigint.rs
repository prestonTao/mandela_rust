extern crate mandela_rust;

use mandela_rust::core;

// #[test]
// fn it_works() {
//     assert_eq!(4, adder::add_two(2));
// }

#[test]
fn bigExample(){

let temp = core::utils::big::int::BigInt::newInt(-1775);
	println!("{:?}", temp.getBytes());

	let mut temp = core::utils::big::int::BigInt::newInt(1);
	temp.lsh(8);
	println!("{}", temp.format(2));
	temp.rsh(16);
	println!("{}", temp.format(2));

	let mut temp = core::utils::big::parse::ParseInt("00010110011111111000100100101111110111001111100110000010011001010101100111101000111000011101011101110100111111110010011110000010110110011011111010000001100000001000000100001101111111001001110110000011001111111000101011101000000010000100110111110010101011011111111010001111110110010001110010011101111100111111011111001000100001100011101000010111011000101101101110010001000111110010001001100101001101101101100011100101011101011001011001010111101001101101000101000101010101110001101100011010110001011010111011011000".to_string(), 2);
	println!("temp {:?}", temp.getBytes());
	temp.rsh(512);
	println!("temp1 {}", temp.format(2));


	let mut src = core::utils::big::parse::ParseInt("011011101111".to_string(), 2);
	let mut dst = core::utils::big::parse::ParseInt("01010".to_string(), 2);

	//解析
	//正数
	src = core::utils::big::parse::ParseInt("011011101111".to_string(), 2);
	src = core::utils::big::parse::ParseInt("1775".to_string(), 10);
	src = core::utils::big::int::BigInt::newInt(1775);
	//负数
	src = core::utils::big::parse::ParseInt("111011101111".to_string(), 2);
	src = core::utils::big::parse::ParseInt("-1775".to_string(), 10);
	src = core::utils::big::int::BigInt::newInt(-1775);
	println!("格式化 {}", src.format(10));
	println!("格式化 {}", src.format(2));


	
	src = core::utils::big::parse::ParseInt("0111111111".to_string(), 2);
	src.lsh(9);
	println!("左移 {}", src.format(2));


	src.rsh(1);
	println!("右移 {}", src.format(2));


	src.not();
	println!("取反 {}", src.format(2));

	let mut dstBigInt = core::utils::big::parse::ParseInt("011".to_string(), 2);
	src.xor(&dstBigInt);
	println!("异或 {}", src.format(2));


	src = core::utils::big::parse::ParseInt("-10".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("-11".to_string(), 10);
	println!("对比 {}", src.cmp(&dst));

	src = core::utils::big::parse::ParseInt("-10".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("-1".to_string(), 10);
	src.add(&dst);
	println!("加法 {}", src.format(10));

	src = core::utils::big::parse::ParseInt("18".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("10".to_string(), 10);
	src.sub(&dst);
	println!("减法 {}", src.format(10));


	src = core::utils::big::int::BigInt::newInt(9998);
	dst = core::utils::big::int::BigInt::newInt(9907);
	src.mul(&dst);
	println!("乘法 {}", src.format(10));


	src = core::utils::big::parse::ParseInt("9997".to_string(), 10);
	dst = core::utils::big::parse::ParseInt("10".to_string(), 10);
	src.div(&dst);
	println!("除法 {}", src.format(10));


	src = core::utils::big::int::BigInt::newInt(99999999999999997);
	dst = core::utils::big::int::BigInt::newInt(10);
	src.model(&dst);
	println!("取模 {}", src.format(10));


	src = core::utils::big::parse::ParseInt("-1775".to_string(), 10);
	dst = core::utils::big::int::BigInt::newInt(10);
	src.model(&dst);
	println!("取模 {}", src.format(10));


	let mut bi1 = core::utils::big::parse::ParseInt("5305526635954913487496512607917332448768560051686420921121136606912134860880484640854480807689829618278433318467763688564544064913921449472475328006888080".to_string(), 16);
	let mut bi2 = core::utils::big::parse::ParseInt("4408040337310354913247862118494615552060306278488834430080034004974080933762336006942725024579529063347250566483321699584874656600576649785640460877768043".to_string(), 16);
	let r = bi1.cmp(&bi2);
	println!("{}", r);


	
	let mut bi1 = core::utils::big::parse::ParseInt("2237207350676480905868434767140358938481059234448801495048082336607812732742400550721685573120320961143243392067601226070450240832010412246067257056921674".to_string(), 16);
	let mut bi2 = core::utils::big::parse::ParseInt("4408040337310354913247862118494615552060306278488834430080034004974080933762336006942725024579529063347250566483321699584874656600576649785640460877768043".to_string(), 16);
	let r = bi1.cmp(&bi2);
	println!("{}", r);
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

