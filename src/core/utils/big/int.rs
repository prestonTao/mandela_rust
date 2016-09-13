
use std::cmp;


#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BigInt{
	neg: bool,         //代表数组正负，true=正;false=负
	bytes: Vec<u8>,
}

impl BigInt{
	pub fn New() -> BigInt{
		BigInt{neg: true, bytes: Vec::new()}
	}

	pub fn NewInt(num: i64) -> BigInt{
		let neg = if num>=0 {true}else{false};
		let tempNum = if num>=0{num}else{-num};
		BigInt{neg:neg, bytes: super::base::toVecU8(tempNum as u64)}
	}
	pub fn SetNeg(&mut self, neg: bool){
		self.neg = neg;
	}
	pub fn SetBytes(&mut self, bs: Vec<u8>){
		self.bytes = bs;
	}
	/*
		拷贝一个一样大的数
	*/
	pub fn Copy(&self) -> BigInt{
		BigInt{neg: self.neg, bytes : super::base::copy(&self.bytes)}
	}


	/*
		取反操作
	*/
	pub fn Not(&mut self){
		self.bytes = super::base::not(&self.bytes);
	}

	/*
		向左移位
	*/
	pub fn Lsh(&mut self, num: u64){
		self.bytes = super::base::lsh(&self.bytes, num);
	}
	/*
		向右移位
	*/
	pub fn Rsh(&mut self, num: u64){
		self.bytes = super::base::rsh(&self.bytes, num);
	}

	pub fn Xor(&mut self, dst: &BigInt){
		// let mut dstBytes = dst.bytes;
		if self.neg {
			if dst.neg{
				self.bytes = super::base::xor(&self.bytes, &dst.bytes);
			}else{
				self.bytes = super::base::xor(&self.bytes, &super::base::opposite(&dst.bytes));
			}
		}else{
			if dst.neg{
				self.bytes = super::base::xor(&super::base::opposite(&self.bytes), &dst.bytes);
			}else{
				self.bytes = super::base::xor(&super::base::opposite(&self.bytes), &super::base::opposite(&dst.bytes));
			}
		}
	}

	pub fn Cmp(&self, dst: &BigInt) -> i8{
		if self.neg == dst.neg{
			if self.neg{
				return super::base::cmp(&self.bytes, &dst.bytes);
			}else{
				return -super::base::cmp(&self.bytes, &dst.bytes);
			}
		}else{
			if self.neg{
				return 1;
			}else{
				return -1;
			}
		}
	}

	/*
		加法运算：将一个整数用二进制表示，其加法运算就是：相异（^）时，本位为1，进位为0；同为1时本位为0，
		进位为1；同为0时，本位进位均为0.所以，不计进位的和为sum = a^b，进位就是arr = a&b,(与sum相加时先
		左移一位，因为这是进位）。完成加法直到进位为0.
	*/
	pub fn Add(&mut self, dst: &BigInt){
		if self.neg == dst.neg{
			self.bytes = super::base::add(&self.bytes, &dst.bytes);
		}else if self.neg{
			match super::base::cmp(&self.bytes, &dst.bytes){
				0 => {
					self.bytes = vec![0];
				},
				-1 => {
					self.neg = false;
					self.bytes = super::base::sub(&dst.bytes, &self.bytes);
				},
				_ => {
					self.bytes = super::base::sub(&self.bytes, &dst.bytes);
				},
			}
		}else{
			match super::base::cmp(&self.bytes, &dst.bytes){
				0 => {
					self.neg = true;
					self.bytes = vec![0];
				},
				-1 => {
					self.neg = true;
					self.bytes = super::base::sub(&dst.bytes, &self.bytes);
				},
				_ => {
					self.bytes = super::base::sub(&self.bytes, &dst.bytes);
				},
			}
		}
	}

	/*
		减法运算
		http://blog.csdn.net/gaoyongxing/article/details/4246956
	*/
	pub fn Sub(&mut self, dst: &BigInt){
		if self.neg == dst.neg{
			match super::base::cmp(&self.bytes, &dst.bytes){
				0 => {
					self.neg = true;
					self.bytes = vec![0];
				},
				1 => {
					if self.neg{
						self.neg = true;
						self.bytes = super::base::sub(&self.bytes, &dst.bytes);
					}else{
						self.neg = false;
						self.bytes = super::base::sub(&self.bytes, &dst.bytes);
					}
				},
				_ => {
					if self.neg{
						self.neg = false;
						self.bytes = super::base::sub(&dst.bytes, &self.bytes);
					}else{
						self.neg = true;
						self.bytes = super::base::sub(&dst.bytes, &self.bytes);
					}
				},
			}
		}else{
			if self.neg{
				self.neg = true;
				self.bytes = super::base::add(&self.bytes, &dst.bytes);
			}else{
				match super::base::cmp(&self.bytes, &dst.bytes){
					0 => {
						self.neg = true;
						self.bytes = vec![0];
					},
					_ => {
						self.neg = false;
						self.bytes = super::base::add(&self.bytes, &dst.bytes);
					},
				}
			}
		}
	}

	/*
		乘法运算
	*/
	pub fn Mul(&mut self, dst: &BigInt){
		let mut neg = true;
		if !self.neg{
			neg = !neg;
		}
		if !dst.neg{
			neg = !neg;
		}
		self.neg = neg;
		self.bytes = super::base::mul(&self.bytes, &dst.bytes);
	}

	/*
		除法运算
	*/
	pub fn Div(&mut self, dst: &BigInt){
		let (value, remainder) = super::base::div(&self.bytes, &dst.bytes);
		self.bytes = value;
	}
	

	/*
		获得余数
	*/
	pub fn Mod(&mut self, dst: &BigInt){
		let (value, remainder) = super::base::div(&self.bytes, &dst.bytes);
		self.bytes = remainder;
	}
	


	/*
		格式化输出
		@format    u8    输出格式，2=2进制，10=10进制，64=base64格式
	*/
	pub fn Format(&self, format: u8) -> String {
		match format{
			2 => {
				if self.neg{
					super::parse::formatTo2(&self.bytes)
				}else{
					super::parse::formatTo2(&super::base::opposite(&self.bytes))
				}
			},
			10 => super::parse::formatTo10(&self.bytes, self.neg),
			16 => super::parse::formatTo16(&self.bytes, self.neg),
			_ => "".to_string(),
		}
	}
}

// impl cmp::PartialEq for BigInt{
// 	    fn eq(&self, other: &BigInt) -> bool{
//     	if self.Cmp(other) == 0{
//     		return true;
//     	}
//     	false
//     }

//     fn ne(&self, other: &BigInt) -> bool { !self.eq(other) }
// }

// impl cmp::Eq for BigInt{

// }


