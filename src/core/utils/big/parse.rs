


/*
	解析字符串
	@s       String    待解析的字符串
	@format  Int       字符串格式，2=2进制，10=10进制，64=base64格式
*/
pub fn ParseInt(s: String, format: u8) -> super::int::BigInt {

	match format {
		2 => parseFor2(s),
		10 => parseFor10(s),
		_ => {
			super::int::BigInt::New()
		}
	}

}

fn parseFor2(s: String) -> super::int::BigInt {
	let mut big = super::int::BigInt::New();
	if s == ""{
		return big;
	}

	// let mut bytes: Vec<u8> = vec![];            //第一个byte是0，需要去掉
	// let mut bytes: Vec<u8> = Vec::<u8>::new();  //第一个byte是0，需要去掉
	let mut bytes: Vec<u8> = Vec::new();           //第一个byte是0，需要去掉

	let mut index = s.len();
	loop{
		if index >= 8 {
			let b = super::parse::getU8(&s[index-8..index]);
			bytes.push(b);
		}else{
			let mut fillStr = "".to_string();
			if &s[0..1] == "0"{
				fillStr = "0".to_string();
			}else{
				big.SetNeg(false);
				fillStr = "0".to_string();
			}
			let mut tempStr = "".to_string();
			for i in 0..(9 - index) {
				tempStr.push_str(&fillStr);
			}

			tempStr.push_str(&s[1..index]);
			let b = super::parse::getU8(&tempStr);
			bytes.push(b);
			break;
		}
		index -= 8;
		if index <= 0 {
			break;
		}
	}
	// bytes.reverse();
	big.SetBytes(bytes);
	big
}


fn parseFor10(s: String) -> super::int::BigInt {
	let mut bigInt = super::int::BigInt::New();
	if s == ""{
		return bigInt;
	}
	let mut newBytes: Vec<u8> = Vec::new();
	let length = s.len();
	let mut index = 0;
	let mut temp: Vec<u8> = vec![1];
	loop{
		if index >= length-1{
			if &s[0..1] == "-"{
				bigInt.SetNeg(false);
				break;
			}
		}
		match &s[(length-index-1)..(length-index)]{
			"0" => {
			},
			"1" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![1]));
			},
			"2" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![2]));
			},
			"3" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![3]));
			},
			"4" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![4]));
			},
			"5" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![5]));
			},
			"6" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![6]));
			},
			"7" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![7]));
			},
			"8" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![8]));
			},
			"9" => {
				newBytes = super::base::add(&newBytes, &super::base::mul(&temp, &vec![9]));
			},
			_ => {
				return super::int::BigInt::New();
			},
		}
		index += 1;
		if index >= length{
			break;
		}
		temp = super::base::mul(&temp, &vec![10]);
	}
	bigInt.SetBytes(newBytes);
	bigInt
}



/*
	将byte数组转化为二进制字符串
*/
pub fn formatTo2(bytes: &Vec<u8>) -> String {
	let mut strs: Vec<String> = Vec::new();
	for byte in bytes{
		for i in 0..8{
			if (byte & (1 << i)) == 0{
				strs.push("0".to_string());
			}else{
				strs.push("1".to_string());
			}
		}
	}
	strs.reverse();
	let mut bitstr: String = "".to_string();
	for one in strs{
		bitstr.push_str(&one);
	}
	if bitstr == "".to_string(){
		bitstr = "0".to_string();
	}
	bitstr
}

/*
	通过8位二进制字符串转化为一个u8整数
*/
pub fn getU8(s: &str) -> u8 {
	let mut num: u8 = 0;
	for (i, one) in s.chars().enumerate(){
		if one == '1'{
			let temp = 1 << (7-i);
			num = num + temp;
		}
	}
	num
}

/*
	将byte数组转化为十进制字符串
*/
pub fn formatTo10(bytes: &Vec<u8>, neg: bool) -> String{
	let mut strs: Vec<String> = Vec::new();
	let mut dst: Vec<u8> = vec![10];
	let mut remaining: Vec<u8> = super::base::copy(&bytes);
	loop{
		match super::base::cmp(&remaining, &dst) {
			0 => {
				break;
			},
			-1 => {
				if remaining.len() > 0{
					strs.push(remaining[0].to_string());
				}
				break;
			},
			_ => {
				let (value, rem) = super::base::div(&remaining, &dst);
				strs.push(rem[0].to_string());
				remaining = value;
			},
		}
	}
	let mut bitstr: String = "".to_string();
	if strs.len() == 0{
		strs.push("0".to_string());
	}else{
		if !neg{
			bitstr.push_str(&"-".to_string());
		}
	}
	strs.reverse();
	
	for one in strs{
		bitstr.push_str(&one);
	}
	bitstr
}