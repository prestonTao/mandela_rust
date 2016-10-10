
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;


struct SessionBase{
    name: String,
    // attrbutes: HashMap<String, Any>,
}


pub trait Session{
    fn sendBytes();
    fn sendString();
    // Send(msgID, opt, errcode uint32, cryKey []byte, data *[]byte) error
	// Close()
	// Set(name string, value interface{})
	// Get(name string) interface{}
	// GetName() string
	// SetName(name string)
	// GetRemoteHost() string
}

struct SessionStore<T: Session>{
    sessionmap: RwLock<HashMap<String, Arc<T>>>,
}

impl<T: Session> SessionStore<T>{
    fn addSession(&mut self, name: String, session: T){
        match self.sessionmap.write() {
            Err(e) => {},
            Ok(mut x) => {
                x.insert(name, Arc::new(session));
            },
        };
    }
    fn getSession(self, name: String) -> Option<Arc<T>>{
        let map = match self.sessionmap.read() {
            Err(e) => return None ,
            Ok(x) => x 
        };
        match map.get(&name){
            None => return None,
            Some(x) => return Some(x.clone()),
        }
    }
}