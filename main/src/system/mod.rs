use server::Service;

pub struct UsersData {
    users: Vec<User>,
    ranking: Vec<usize>
}

struct User {
    name: String,
    id: u64,
    score: i32
}

impl UsersData {
    pub fn new() -> UsersData {
        UsersData {
            users: Vec::new(),
            ranking: Vec::new()
        }
    }

    pub fn response(&mut self, uri: &str) -> Result<String, ()> {
        Ok(String::from("Hello by 10App"))
    }
}

impl Service for UsersData {
    fn response(&mut self, uri: &str) -> Result<String, ()> {
        Ok(String::from("<h1>Hello, World!</h1>by system"))
    }

    fn save(&self) {

    }
}