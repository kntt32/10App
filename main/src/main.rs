use server::Server;
mod system;
use system::UsersData;

fn f(uri: &str) -> Result<String, ()> {
    Ok(String::from(format!("<h1>Hello, World!</h1>{}", uri)))
}

fn main() {
    let mut users_data = UsersData::new();

    let mut server = Server::new("127.0.0.1:80", Box::new(UsersData::new()));

    server.response_loop();
}
