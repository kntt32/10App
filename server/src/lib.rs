use std::net::TcpListener;
use std::net::TcpStream;
use std::error::Error;
use std::io::Read;
use std::io::Write;


type ResponseFn = fn(&str) -> Result<String, ()>;


pub struct Server {
    tcp_listener: TcpListener,
    response: ResponseFn
}

trait Service {
    fn response(&mut self, uri: &str) -> Result<String, ()>;

    fn save(&self) {

    }
}


impl Server {
    pub fn new(addr: &str, response: ResponseFn) -> Server {
        Server {
            tcp_listener: TcpListener::bind(addr).expect("failed to bind network"),
            response: response
        }
    }

    
    pub fn response_loop(&mut self) {
        fn get_http_message(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
            const DEFAULT_BUFF_SIZE: usize = 10000;

            let mut buff = Vec::new();
            buff.resize(DEFAULT_BUFF_SIZE, 0u8);
            stream.read(&mut buff[0 .. DEFAULT_BUFF_SIZE])?;
            Ok(String::from_utf8(buff)?)
        }

        fn get_uri(request: &str) -> Result<&str, ()> {
            let splitted_str: Vec<&str> = request.splitn(3, ' ').collect();

            if 2 <= splitted_str.len() {
                Ok(splitted_str[1])
            }else {
                Err(())
            }
        }


        for stream in self.tcp_listener.incoming() {
            let mut stream = stream.unwrap();

            if let Ok(request) = get_http_message(&mut stream) {
                if let Ok(uri) = get_uri(&request) {
                    let response = (self.response)(uri);

                    let response_http_message = 
                    if let Ok(response) = response {
                        "HTTP/1.1 200 OK\nContent-Type: text/html\n\n".to_string() + &response
                    }else {
                        "HTTP/1.1 404 Not Found\nContent-Type: text/html\n\n<h1>Not Found</h1>".to_string()
                    };

                    

                    if let Ok(_) = stream.write_all(&response_http_message.into_bytes()) {}
                }
            }
        }
    }
}