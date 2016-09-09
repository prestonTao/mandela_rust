extern crate time;
extern crate core;
extern crate crypto;
use core::utils::big::int::BigInt;

struct IDInfo{
	ID: BigInt, //ID
	CreateTime: String, //创建时间
	Domain: String,      //域名
	Name: String,       //姓名
	Email: String,      //email
	SuperNodeId: String, //创建者节点id
}


pub struct Node{
	IDInfo: IDInfo,                       //节点id信息，id字符串以16进制显示
	IsSuper: bool,                    //是不是超级节点，超级节点有外网ip地址，可以为其他节点提供代理服务
	Addr: String,                        //外网ip地址
	TcpPort: i32,                     //TCP端口
	UdpPort: i32,                     //UDP端口
	LastContactTimestamp: time::Tm,   //最后检查的时间戳
}

impl Node{
	pub fn New() -> Node{
		let big = BigInt::New();
		let idInfo = IDInfo{
			ID:     big,
			CreateTime: "".to_string(), //创建时间
			Domain: "".to_string(),      //域名
			Name: "".to_string(),       //姓名
			Email: "".to_string(),      //email
			SuperNodeId: "".to_string(), //创建者节点id
		};
		Node{
			IDInfo:idInfo,
			IsSuper: false,                    //是不是超级节点，超级节点有外网ip地址，可以为其他节点提供代理服务
			Addr: "".to_string(),                        //外网ip地址
			TcpPort: 1,                     //TCP端口
			UdpPort: 1,                     //UDP端口
			LastContactTimestamp: time::now(),   //最后检查的时间戳
		}
	}
	pub fn GetID(&self) -> &BigInt{
		&self.IDInfo.ID
	}
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
	// pub fn New(name: &str, email: &str, domain: &str, superNodeId: &str) -> IDInfo{

	// }
}