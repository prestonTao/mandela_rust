#![warn(rust_2018_idioms)]

extern crate mandela_rust;
use mandela_rust::core;
// use std::thread;


// extern crate futures;
// extern crate tokio_core;
// use futures::stream::Stream;
// use tokio_core::reactor::Core;
// use tokio_core::net::TcpListener;


// extern crate mio;
extern crate net2;


// fn main() {


// 	example();

// }


async fn example(){

	let mut engine = core::engine::engine::Engine::new();
	engine.registerMSG(1, login);
	engine.linsten("0.0.0.0:19981");
	thread::sleep_ms(9999999);


}

fn login(){

}





use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        for n in (0..100) {
            println!("1111");
            // thread::sleep_ms(std::time::Duration::MILLISECOND);
            thread::sleep_ms(100);
        }
    });

    tokio::spawn(async move {
        for _n in (0..100) {
            println!("2222");
            // thread::sleep_ms(std::time::Duration::MILLISECOND);
            thread::sleep_ms(100);
        }
    });

	// tokio::spawn(example);
	println!("haha");
	




    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
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



