
extern crate net2;
use std::error::Error;
use std::thread;
use std::io;
use std::io::Read;
use std::net::TcpStream;
use std::time;



// macro_rules! t {
//     ($e:expr) => (match $e {
//         Ok(e) => e,
//         Err(e) => panic!("{} failed with: {}", stringify!($e), e),
//     })
// }


pub struct Engine{
    router: super::router::Router,
}

impl Engine{
    pub fn new() -> Engine{
        Engine{router: super::router::Router::new()}
    }

    pub fn registerMSG(&mut self, msgID: usize, handle: super::router::Handle){
        self.router.addHandle(msgID, handle);
    }

    pub fn linsten(&self, addr: &str) -> io::Result<net2::TcpBuilder> {
        let b = match net2::TcpBuilder::new_v4(){
            Err(e) => {return Err(e)},
            Ok(x) => x
        };
        match b.bind(addr) {
            Err(e) => {return Err(e)},
            Ok(x) => x
        };
        let listener = match b.listen(200) {
            Err(e) => {return Err(e)},
            Ok(x) => x
        };
        let t = thread::spawn(move || {
            loop {
                // let bi = |s|{self.buildServerConn(s);};
                match listener.accept() {
                    Err(e) => {break;},
                    Ok(x) => {
                        // let s = x.0;
                        thread::spawn(move ||{
                            let mut s = x.0;
                            let mut b = [0; 20];
                            s.set_read_timeout(Some(time::Duration::new(10, 0)));
                            s.read(&mut b);
                            println!("{:?}", b);
                            b = [0; 20];
                            
                            s.read(&mut b);
                            println!("{:?}", b);
                        });
                    },
                };
            }
        });
        Ok(b)
    }
    // pub fn buildServerConn(self, s: &mut TcpStream){
    //     let mut b = [0; 10];
    //     s.read(&mut b);
    //     // assert_eq!(b, [1, 2, 3, 0]);
    //     println!("{:?}", b);
    //     s.read(&mut b);
    //     // assert_eq!(b, [1, 2, 3, 0]);
    //     println!("{:?}", b);
    // }
}

