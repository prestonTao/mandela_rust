


/*
	复制
*/
pub fn copy(src: &Vec<u8>) -> Vec<u8>{
	let mut num: Vec<u8> = Vec::new();
	for one in src{
		num.push(*one);
	}
	num
}

/*
	获得正负号
*/
pub fn getNeg(src: &Vec<u8>) -> bool {
	if (src[src.len()-1] & 0b10000000) == 0{
		true
	}else{
		false
	}
}

//向左移位
pub fn lsh(src: &Vec<u8>, num: u64) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	let indexMajor = num / 8;
	let indexMinor = (num % 8) as u8;
	//低位补零
	for i in 0..indexMajor{
		newBytes.push(0);
	}
	
	let len = src.len();
	let mut index = 0;
	let mut fill: u8 = 0;
	for one in src{
		let (newByte, overflow) = lsh_byte(*one, indexMinor, fill);
		newBytes.push(newByte);
		fill = overflow;
	}
	if fill != 0{
		newBytes.push(fill);
	}
	cleanZero(&mut newBytes);
	newBytes
}

/*
	向左移位
	@num    u8    需要位移的byte
	@left   u8    移几位
	@fill   u8    补位值
	@return u8    移位后的值
	@return u8    溢出的值
*/
fn lsh_byte(num: u8, left: u8, fill: u8) -> (u8, u8) {
	//超出处理范围
	if left > 8 {
		return (0, 0);
	}
	let mut temp: u16 = num as u16;
	temp = temp << left;

	//取溢出位
	let overflow: u8 = ((0b1111111100000000 & temp) >> 8) as u8;
	let mut newByte: u8 = temp as u8;
	newByte = newByte | fill; //填充溢出位
	(newByte, overflow)
}

/*
	向右位移
*/
pub fn rsh(src: &Vec<u8>, num: u64) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	let indexMajor = (num / 8) as usize;
	let indexMinor = num % 8;


	let len = src.len();
	let mut index = len;
	let mut fill: u8 = 0;
	loop{
		let (newByte, overflow) = rsh_byte(src[index-1], indexMinor, fill);
		newBytes.push(newByte);
		fill = overflow;
		index -= 1;
		if index <= indexMajor {
		// if (index - indexMajor -1) <= 0 {
			break;
		}
	}
	newBytes.reverse();
	cleanZero(&mut newBytes);
	newBytes
}

/*
	向右移位
	@num    u8    需要位移的byte
	@right  u8    移几位
	@fill   u8    补位值
	@return u8    移位后的值
	@return u8    溢出的值
*/
fn rsh_byte(num: u8, right: u64, fill: u8) -> (u8, u8){
	//超出处理范围
	if right > 8 {
		return (0, 0);
	}
	let mut temp: u16 = num as u16;
	temp = temp << 8;
	temp = temp >> right;
	//取溢出位
	let overflow: u8 = (255 & temp) as u8;
	let mut newByte: u8 = (temp >> 8) as u8;
	newByte = newByte | fill; //填充溢出位
	(newByte, overflow)
}

/*
	取反码操作
*/
pub fn not(src: &Vec<u8>) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	for one in src{
		newBytes.push(!one);
	}
	cleanZero(&mut newBytes);
	newBytes

}


/*
	异或操作
*/
pub fn xor(src: &Vec<u8>, bytes: &Vec<u8>) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	let mut srcBytes: Vec<u8> = copy(src);
	let mut dstBytes: Vec<u8> = copy(bytes);
	let mut maxLen = 0;
	if src.len() > bytes.len(){
		maxLen = src.len();
		dstBytes = fillComplement(&dstBytes, maxLen, true);
	}else{
		maxLen = bytes.len();
		srcBytes = fillComplement(&srcBytes, maxLen, true);
	}

	let mut index = 0;
	loop{
		newBytes.push(srcBytes[index] ^ dstBytes[index]);
		index += 1;
		if index >= maxLen{
			break;
		}
	}
	cleanZero(&mut newBytes);
	newBytes
}

/*
	对比正数大小
*/
pub fn cmp(src: &Vec<u8>, dst: &Vec<u8>) -> i8{
	let mut srcBytes: Vec<u8> = copy(src);
	let mut dstBytes: Vec<u8> = copy(dst);
	cleanZero(&mut srcBytes);
	cleanZero(&mut dstBytes);
	if srcBytes.len() > dstBytes.len(){
		return 1;
	}else if srcBytes.len() < dstBytes.len(){
		return -1;
	}else{
		let mut index = 0;
		for one in srcBytes{
			let dstOne = dstBytes[index];
			if one > dstOne{
				return 1;
			}else if one < dstOne{
				return -1;
			}
			index += 1;
		}
		return 0;
	}
}


/*
	加法操作
*/
pub fn add(src: &Vec<u8>, bytes: &Vec<u8>) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	let mut srcBytes: Vec<u8> = copy(src);
	let mut dstBytes: Vec<u8> = copy(bytes);
	let mut maxLen = 0;
	if src.len() > bytes.len(){
		maxLen = src.len();
		dstBytes = fillComplement(&dstBytes, maxLen, false);
	}else{
		maxLen = bytes.len();
		srcBytes = fillComplement(&srcBytes, maxLen, false);
	}

	let mut index = 0;
	let mut sum = 0;
	loop{
		let (value, over) = add_byte(srcBytes[index], dstBytes[index], sum);
		newBytes.push(value);
		sum = over;

		index += 1;
		if index >= maxLen{
			break;
		}
	}
	if sum != 0{
		newBytes.push(sum);
	}
	cleanZero(&mut newBytes);
	newBytes
}

/*
	加法操作
	@src    u8    被加数
	@dst    u8    加数
	@sum    u8    进位累加
	@return u8    加法后的值
	@return u8    进位值
*/
fn add_byte(src: u8, dst: u8, sum: u8) -> (u8, u8){
	let mut jw: u16 = (src&dst) as u16;
 	let mut jg: u16 = (src^dst) as u16;
 	while jw !=0 {
		let t_a: u16 = jg;
		let t_b: u16 = jw<<1;
		jw=t_a&t_b;
		jg=t_a^t_b;
	}
	//加上累加值
	jw = jg& (sum as u16);
 	jg = jg^ (sum as u16);
 	while jw !=0 {
		let t_a: u16 = jg;
		let t_b: u16 = jw<<1;
		jw=t_a&t_b;
		jg=t_a^t_b;
	}
	(jg as u8, (jg>>8) as u8)
}

/*
	减法操作
*/
pub fn sub(src: &Vec<u8>, dst: &Vec<u8>) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	let mut srcBytes: Vec<u8> = copy(src);
	srcBytes.push(0);
	let mut dstBytes: Vec<u8> = opposite(&copy(dst));
	dstBytes.push(255);

	let mut maxLen = 0;
	if srcBytes.len() > dst.len(){
		maxLen = srcBytes.len();
		dstBytes = fillComplement(&dstBytes, maxLen, true);
	}else{
		maxLen = dstBytes.len();
		srcBytes = fillComplement(&srcBytes, maxLen, false);
	}

	
	let mut sum = 0;
	let mut index = 0;
	for srcOne in srcBytes{
		let (value, over) = add_byte(srcOne, dstBytes[index], sum);
		newBytes.push(value);
		sum = over;
		index += 1;
	}

	cleanZero(&mut newBytes);
	newBytes
	//获取减数的相反数
	// add(src, &opposite(dst))
}

/*
	乘法
*/
pub fn mul(src: &Vec<u8>, dst: &Vec<u8>) -> Vec<u8>{
	let mut neg = true;
	let mut srcBytes: Vec<u8> = copy(src);
	let mut dstBytes: Vec<u8> = copy(dst);
	if src[src.len()-1] >= 0b10000000{
		neg = !neg;
		srcBytes = opposite(&srcBytes);
	}
	if dst[dst.len()-1] >= 0b10000000{
		neg = !neg;
		dstBytes = opposite(&dstBytes);
	}

	// let mut value: Vec<u8> = Vec::new();
	let mut count: u64 = (dstBytes.len() * 8) as u64;
	// for i in &dstBytes{
	// 	count = toU64(&add(&toVecU8(count), &vec![8]));
	// }

	let mut temp: Vec<u8> = lsh(&vec![1], count);
	let mut sum: Vec<u8> = Vec::new();
	let mut augend: Vec<u8> = lsh(&src, count);
	loop{
		temp = rsh(&temp, 1);
		augend = rsh(&augend, 1);
		match cmp(&and(&dst, &temp), &vec![0]) {
			0 => {
			},
			_ => {
				sum = add(&sum, &augend);
			},
		}
		// sum = add(&sum, &and(&temp, ));
		count -= 1;
		if count == 0{
			break;
		}
	}
	
	sum
}

/*
	算法思想
	00001001/00000011=00000011
	被除数是00001001(9)
	除数是00000011(3)
	商是00000011(3)
	余数是00000000(0)

	算法思想是：
	1、将被除数高位移入Temp中(高位开始按顺序移入Temp中)
	2、将Temp减去除数
	3、如果小于0，则置商值低位为0
	     如果大于等于0，则置商值低位为1，并将相减的结果放入Temp中
	4、将商值左移1位
	5、判断是否循环完成（一共循环8次）
	6、没有则跳到步骤1继续执行
	7、完成则存入商值和余数（Temp）
*/
pub fn div(src: &Vec<u8>, dst: &Vec<u8>) -> (Vec<u8>, Vec<u8>){
	let mut srcBytes = copy(src);
	let mut count = srcBytes.len() * 8;
	let mut value: Vec<u8> = vec![0];
	let mut temp: Vec<u8> = vec![0];
	for i in 0..count{
		if (srcBytes[srcBytes.len()-1] & 0b10000000) >0 {
			temp[0] = temp[0] | 1;
			srcBytes = lsh(&srcBytes, 1);
			srcBytes.pop();
		}else{
			srcBytes = lsh(&srcBytes, 1);
		}
		if cmp(&temp, &dst) >= 0{
			value[0] = value[0] | 1;
			temp = sub(&temp, &dst);
		}
		temp = lsh(&temp, 1);
		value = lsh(&value, 1);
	}
	value = rsh(&value, 1);
	temp = rsh(&temp, 1);
	(value, temp)
}




/*
	清除高位上的零
*/
fn cleanZero(src: &mut Vec<u8>){
	loop{
		if src.len() <= 1{
			return
		}
		if src[src.len()-1] == 0{
			src.pop();
		}else{
			return
		}
	}
}





/*
	填充二进制高位补0
	@bool    isneg    是否有符号
*/
fn fillComplement(src: &Vec<u8>, count: usize, isneg: bool) -> Vec<u8> {

	let mut newBytes: Vec<u8> = Vec::new();
	for one in src{
		newBytes.push(*one);
	}
	if src.len() >= count {
		return newBytes;
	}
	if src.len() == 0{
		for i in 0..count{
			newBytes.push(0);
		}
		return newBytes;
	}
	let fillCount = count - src.len();
	if isneg{
		if (src[src.len()-1] & 0b10000000) == 0{
			for i in 0..fillCount{
				newBytes.push(0);
			}
		}else{
			for i in 0..fillCount{
				newBytes.push(255);
			}
		}
	}else{
		for i in 0..fillCount{
			newBytes.push(0);
		}
	}
	

	newBytes
}


/*
	取一个数的相反数，负数变正，正数变负，负数用补码表示
*/
pub fn opposite(src: &Vec<u8>) -> Vec<u8> {
	let mut newBytes: Vec<u8> = Vec::new();
	//负数变正
	if src[src.len()-1] >= 0b10000000{
		//补码减1是反码
		let mut temp: Vec<u8> = Vec::new();
		let mut done = false;
		for one in src{
			if done{
				temp.push(*one);
				continue;
			}
			if *one == 0{
				temp.push(254);
			}else{
				temp.push(one - 1);
				done = true;
			}
		}
		for one in temp {
			newBytes.push(!one);
		}
	}else{
		//正数变负
		//获取反码
		let mut temp: Vec<u8> = Vec::new();
		for one in src{
			temp.push(!one);
		}
		//反码加1就是补码
		let mut dst: u8 = 1;
		let mut sum = 0; //累加值
		for one in temp{
			let (value, over) = add_byte(one, dst, sum);
			newBytes.push(value);
			sum = over;
			dst = 0;
		}
	}
	cleanZero(&mut newBytes);
	newBytes
}

//将8个u8类型转化为一个u64
pub fn toU64(src: &Vec<u8>) -> u64{
	let mut sum: u64 = 0;
	let mut count = 0;
	for one in src{
		sum += (*one as u64) << (8*count);
		count += 1;
		if count >= 7{
			break;
		}
	}
	sum
}

pub fn toVecU8(src: u64) -> Vec<u8>{
	let mut bytes: Vec<u8> = Vec::new();
	bytes.push(src as u8);
	for i in 1..8{
		bytes.push((src >> (8*i)) as u8);
	}
	cleanZero(&mut bytes);
	bytes
}

/*
	与操作
	z = x & y;
*/
pub fn and(src: &Vec<u8>, dst: &Vec<u8>) -> Vec<u8>{
	let mut newBytes: Vec<u8> = Vec::new();
	let mut srcBytes: Vec<u8> = copy(src);
	let mut dstBytes: Vec<u8> = copy(dst);
	let mut maxLen = 0;
	if src.len() > dst.len(){
		maxLen = src.len();
		dstBytes = fillComplement(&dstBytes, maxLen, true);
	}else{
		maxLen = dst.len();
		srcBytes = fillComplement(&srcBytes, maxLen, true);
	}

	for i in 0..maxLen{
		newBytes.push(srcBytes[i] & dstBytes[i]);
	}
	cleanZero(&mut newBytes);
	newBytes
}