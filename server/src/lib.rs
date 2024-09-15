use std::net::TcpListener;
use std::net::TcpStream;
use std::error::Error;
use std::io::Read;
use std::io::Write;

//mod uri;

static ADMIN_PAGE_URI: &str = "/admin240shuma/";
static ADMIN_PAGE_HTML: &str = "
<!DOCTYPE html>
<meta charset=\"utf-8\">
<html>
    <head>
        <style type=\"text/css\">
        </style>

        <script>
        function shutdown() {
            if(confirm(\"シャットダウンしますか?\")) {
                location.href = \"/admin240shuma/shutdown\";
            }
        }
        </script>
    </head>

    <body>
        <h1>AdminPage</h1>
        <button onclick=\"shutdown()\">Shutdow</button>
    </body>
</html>
";

static SHUTDOWN_URI: &str = "/admin240shuma/shutdown/";
static SHUTDOWN_HTML: &str = "<h1>System Shutdowned</h1>";


pub struct Server {
    tcp_listener: TcpListener,
    service: Box<dyn Service>
}

pub trait Service {
    fn response(&mut self, uri: &str) -> Result<String, ()>;

    fn save(&self);
}


impl Server {
    pub fn new(addr: &str, service: Box<dyn Service>) -> Server {
        Server {
            tcp_listener: TcpListener::bind(addr).expect("failed to bind network"),
            service: service
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

        fn get_uri(request: &str) -> Result<String, ()> {
            let splitted_str: Vec<&str> = request.splitn(3, ' ').collect();

            if 2 <= splitted_str.len() {
                let mut uri_string = splitted_str[1].to_string();
                if !uri_string.ends_with('/') {
                    uri_string.push('/');
                }
                Ok(uri_string)
            }else {
                Err(())
            }
        }

        enum ResponseType {
            Ok(String),
            NotFound(String),
            Shutdown(String)
        }

        fn response(service: &mut Box<dyn Service>, uri: &str) -> ResponseType {
            if uri == ADMIN_PAGE_URI {
                ResponseType::Ok(ADMIN_PAGE_HTML.to_string())
            }else if uri == SHUTDOWN_URI {
                service.save();
                ResponseType::Shutdown(SHUTDOWN_HTML.to_string())
            }else {
                let response = service.response(uri);

                if let Ok(response_string) = response {
                    ResponseType::Ok(response_string.to_string())
                }else {
                    ResponseType::NotFound("<h1>Not Found</h1>".to_string())
                }
            }
        }

        for stream in self.tcp_listener.incoming() {
            let mut stream = stream.unwrap();

            if let Ok(request) = get_http_message(&mut stream) {
                if let Ok(uri) = get_uri(&request) {
                    println!("request: {}", uri);
                    let wrapped_response = response(&mut self.service, &uri);
                    
                    let response_http_message = match wrapped_response {
                        ResponseType::Ok(msg) => "HTTP/1.1 200 OK\nContent-Type: text/html\n\n".to_string() + &msg,
                        ResponseType::NotFound(msg) => "HTTP/1.1 404 NotFound\nContent-Type: text/html\n\n".to_string() + &msg,
                        ResponseType::Shutdown(msg) => {
                            let shutdown_message = "HTTP/1.1 200 OK\nContent-Type: text/html\n\n".to_string() + &msg;
                            if let Ok(_) = stream.write_all(&shutdown_message.into_bytes()) {}
                            std::process::exit(0);
                        }
                    };

                    if let Ok(_) = stream.write_all(&response_http_message.into_bytes()) {}
                }
            }
        }
    }
}