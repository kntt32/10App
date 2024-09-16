use server::Server;
mod system;
use system::UsersData;

fn main() {
    let mut server = Server::new("127.0.0.1:80", Box::new(UsersData::new()));

    server.response_loop();
}
