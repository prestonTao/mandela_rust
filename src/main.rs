extern crate mandela_rust;
use mandela_rust::core;
use std::thread;


// extern crate futures;
// extern crate tokio_core;
// use futures::stream::Stream;
// use tokio_core::reactor::Core;
// use tokio_core::net::TcpListener;


// extern crate mio;
extern crate net2;


fn main() {


	example();

}


fn example(){

	let mut engine = core::engine::engine::Engine::new();
	engine.registerMSG(1, login);
	engine.linsten("0.0.0.0:19981");
	thread::sleep_ms(9999999999999);


}

fn login(){

}


// fn example2(){


// 	println!("---------------------------------\n");



//     let mut core = Core::new().unwrap();
//     let address = "0.0.0.0:8080".parse().unwrap();
//     let listener = TcpListener::bind(&address, &core.handle()).unwrap();

//     let addr = listener.local_addr().unwrap();
//     println!("Listening for connections on {}", addr);

//     let clients = listener.incoming();
//     let welcomes = clients.and_then(|(socket, _peer_addr)| {
//         tokio_core::io::write_all(socket, b"Hello!\n")
//     });
//     let server = welcomes.for_each(|(_socket, _welcome)| {
//         Ok(())
//     });

//     core.run(server).unwrap();

// }



