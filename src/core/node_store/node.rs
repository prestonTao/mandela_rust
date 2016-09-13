extern crate time;
extern crate core;
extern crate crypto;
extern crate rustc_serialize;

use core::utils::big::parse;
use core::utils::big::int::BigInt;
use self::crypto::sha3;
use self::crypto::digest::Digest;
use self::rustc_serialize::hex::ToHex;



// #[derive(Hash, Eq, PartialEq, Debug)]
// #[derive(Debug)]
pub struct Node{
	IDInfo: IDInfo,                       //节点id信息，id字符串以16进制显示
	IsSuper: bool,                    //是不是超级节点，超级节点有外网ip地址，可以为其他节点提供代理服务
	Addr: String,                        //外网ip地址
	TcpPort: u16,                     //TCP端口
	UdpPort: u16,                     //UDP端口
	LastContactTimestamp: time::Tm,   //最后检查的时间戳
}

impl Node{
	pub fn new(id: IDInfo, isSuper: bool, addr: &str, tPort: u16, uPort: u16) -> Node{
		Node{
			IDInfo:id,
			IsSuper: isSuper,                    //是不是超级节点，超级节点有外网ip地址，可以为其他节点提供代理服务
			Addr: addr.to_string(),              //外网ip地址
			TcpPort: tPort,                      //TCP端口
			UdpPort: uPort,                      //UDP端口
			LastContactTimestamp: time::now(),   //最后检查的时间戳
		}
	}
	pub fn getID(&self) -> &BigInt{
		&self.IDInfo.ID
	}
}

// #[derive(Debug)]
pub struct IDInfo{
	pub ID: BigInt, //ID
	CreateTime: String, //创建时间
	Domain: String,      //域名
	Name: String,       //姓名
	Email: String,      //email
	SuperNodeId: String, //创建者节点id
}

/*
	userName      用户名，最大长度100 ""
	email         email，最大长度100 "example@mail.com"
	domain        域名地址，最大长度100 "任意utf-8字符串"
	superNodeId   超级节点id，128位定长 ""
	superNodeKey  超级节点密钥
	rerutn idInfo
	return err
	
*/

impl IDInfo{
	pub fn new(name: &str, email: &str, domain: &str, superNodeId: &str) -> IDInfo {
		let input = name.to_string() + email + domain + superNodeId;
		// let sha = Sha3::sha3_512();
		let mut sha = sha3::Sha3::sha3_512();
		sha.input(&input.as_bytes());

		let mut out_str = vec![0u8; 64];
        sha.result(&mut out_str);
		let bi = parse::ParseInt(out_str.to_hex(), 16);


		IDInfo{
			ID: bi,
			//TODO 时间格式
			CreateTime: "".to_string(),           //创建时间
			Domain: domain.to_string(),           //域名
			Name: name.to_string(),               //姓名
			Email: email.to_string(),             //email
			SuperNodeId: superNodeId.to_string(), //创建者节点id
		}
	}
}