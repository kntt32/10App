use std::net::TcpListener;
use std::net::TcpStream;
use std::error::Error;
use std::io::Read;
use std::io::Write;


const ADMIN_PASSWORD: &str = "shuma240";

const ADMIN_PAGE_URL: &str = "/admin";
static ADMIN_PAGE_AUTH: &str = "
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
    <title>Login</title>
    <head>
        <style>
            .sign {
                font-size: 12px;
                color: #b0b0b0;
            }
            body {
                font-family: sans-serif;
                margin-right: auto;
                margin-left: auto;
                display: grid;
                justify-items: center;
                align-content: start;
            }
            h1 {
                width: auto;
                font-size: 30px;
                margin: 10px;
            }
            h2 {
                width: auto;
                font-size: 25px;
                margin: 10px;
                color: #929292;
            }
            input {
                width: 200px;
                height: 40px;
                font-size: 20px;
                border-radius: 0px;
                border-width: 0px;
                background: #ebebeb;
                margin: 10px;
            }
            button {
                width: 150px;
                height: 40px;
                font-size: 20px;
                border-radius: 0px;
                border-width: 0px;
                background: #dee9ec;
                margin: 10px;
                color: black;
            }
        </style>

        <script>
            function send_password() {
                let object = document.getElementById('password_box');
                location.href = '/admin?' + object.value;
            }
        </script>
    </head>

    <body>
        <h1>AdminPage</h1>
        <input id='password_box' type='password' placeholder='Password'></input>
        <button onclick='send_password()'>Login</button>
        <div class='sign'>built by <a class='sign' href='https://github.com/kntt32/'>kntt32</a></div>
    </body>
</html>
";

static ADMIN_PAGE_HTML: &str = "
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
    <title>AdminPage</title>
    <head>
        <style>
            .sign {
                font-size: 12px;
                color: #b0b0b0;
            }
            body {
                font-family: sans-serif;
                margin-right: auto;
                margin-left: auto;
                display: grid;
                justify-items: center;
                align-content: start;
            }
            h1 {
                width: auto;
                font-size: 30px;
                margin: 10px;
            }
            h2 {
                width: auto;
                font-size: 25px;
                margin: 10px;
                color: #929292;
            }
            input {
                width: 200px;
                height: 40px;
                font-size: 20px;
                border-radius: 0px;
                border-width: 0px;
                background: #ebebeb;
                margin: 10px;
            }
            button {
                width: 150px;
                height: 40px;
                font-size: 20px;
                border-radius: 0px;
                border-width: 0px;
                background: #dee9ec;
                margin: 10px;
                color: black;
            }
        </style>

        <script>
            function service_admin() {
                let path = location.href;
                let splitted_path = path.split('?');
                let query = '';
                if(2 <= splitted_path.length) {
                    query = splitted_path[1];
                }
                location.href = 'admin/service?' + query;
            }

            function save() {
                if(confirm('保存しますか?')) {
                    let path = location.href;
                    let splitted_path = path.split('?');
                    let query = '';
                    if(2 <= splitted_path.length) {
                        query = splitted_path[1];
                    }
                    location.href = '/admin/save_service?' + query;
                }
            }

            function shutdown() {
                if(confirm('シャットダウンしますか?')) {
                    let path = location.href;
                    let splitted_path = path.split('?');
                    let query = '';
                    if(2 <= splitted_path.length) {
                        query = splitted_path[1];
                    }
                    location.href = '/admin/shutdown?' + query;
                }
            }

            function reset() {
                if(confirm('リセットしますか?')) {
                    let path = location.href;
                    let splitted_path = path.split('?');
                    let query = '';
                    if(2 <= splitted_path.length) {
                        query = splitted_path[1];
                    }
                    location.href = '/admin/reset?' + query;
                }
            }
        </script>
    </head>

    <body>
        <h1>AdminPage</h1>
        <button onclick='service_admin()'>ServiceAdmin</button>
        <button onclick='save()'>Save</button>
        <button onclick='shutdown()'>Shutdown</button>
        <button onclick='reset()'>Reset</button>
        <div class='sign'>built by <a class='sign' href='https://github.com/kntt32/'>kntt32</a></div>
    </body>
</html>
";

static JUMP_TO_ADMINPAGE: &str = "
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf8'>
    <head>
        <script>
            window.onload = function() {
                let path = location.href;
                let splitted_path = path.split('?');
                let query = '';
                if(2 <= splitted_path.length) {
                    query = splitted_path[1];
                }

                location.href = '/admin?' + query;
            };
        </script>
    </head>
</html>
";

const SERVICE_ADMIN: &str = "/admin/service";
const SAVESERVICE_URL: &str = "/admin/save_service";
const SHUTDOWN_URL: &str = "/admin/shutdown";
const RESET_URL: &str = "/admin/reset";


pub struct Server {
    tcp_listener: TcpListener,
    service: Box<dyn Service>
}

pub trait Service {
    fn response(&mut self, url: &str, query: &str) -> Result<String, String>;

    fn save(&self);

    fn reset(&mut self);

    fn service_admin(&mut self) -> Result<String, String>;
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
                if uri_string.ends_with('/') {
                    uri_string.pop();
                }
                Ok(uri_string)
            }else {
                Err(())
            }
        }

        fn split_url(uri: &String) -> (&str, &str) {
            let splitted_str: Vec<&str> = uri.splitn(2, "?").collect();

            if 2 <= splitted_str.len() {
                (splitted_str[0], splitted_str[1])
            }else {
                (splitted_str[0], &uri[0 .. 0])
            }
        }

        enum ResponseType {
            Ok(String),
            NotFound(String),
            Shutdown
        }

        fn response(service: &mut Box<dyn Service>, uri: &String) -> ResponseType {
            let (url, query) = split_url(uri);

            match url {
                ADMIN_PAGE_URL => {
                    if query == ADMIN_PASSWORD {
                        ResponseType::Ok(ADMIN_PAGE_HTML.to_string())
                    }else {
                        ResponseType::Ok(ADMIN_PAGE_AUTH.to_string())
                    }
                },
                SERVICE_ADMIN => {
                    if query == ADMIN_PASSWORD {
                        match service.service_admin() {
                            Ok(response) => ResponseType::Ok(response),
                            Err(response) => ResponseType::NotFound(response)
                        }
                    }else {
                        ResponseType::Ok(ADMIN_PAGE_AUTH.to_string())
                    }
                },
                SAVESERVICE_URL => {
                    if query == ADMIN_PASSWORD {
                        service.save();
                        ResponseType::Ok(JUMP_TO_ADMINPAGE.to_string())
                    }else {
                        ResponseType::Ok(ADMIN_PAGE_AUTH.to_string())
                    }
                },
                SHUTDOWN_URL => {
                    if query == ADMIN_PASSWORD {
                        service.save();
                        ResponseType::Shutdown
                    }else {
                        ResponseType::Ok(ADMIN_PAGE_AUTH.to_string())
                    }
                },
                RESET_URL => {
                    if query == ADMIN_PASSWORD {
                        service.reset();
                        ResponseType::Ok(JUMP_TO_ADMINPAGE.to_string())
                    }else {
                        ResponseType::Ok(ADMIN_PAGE_AUTH.to_string())
                    }
                },
                _ => {
                    let response = service.response(url, query);

                    match response {
                        Ok(response_string) => ResponseType::Ok(response_string.to_string()),
                        Err(response_string) => ResponseType::NotFound(response_string)
                    }
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
                        ResponseType::Shutdown => {
                            std::process::exit(0);
                        }
                    };

                    if let Ok(_) = stream.write_all(&response_http_message.into_bytes()) {}
                }
            }
        }
    }
}