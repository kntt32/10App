use server::Service;

static ERROR_PAGE_HTML: &str = "
<!DOCTYPE html>
<html lang=\"ja\">
    <meta charset=\"utf-8\">
    <title>NotFound</title>
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
            }
        </style>
    </head>

    <body>
        <h1>NotFound</h1>
    </body>
</html>
";

static SIGNUP_PAGE_HTML: &str = "
<!DOCTYPE html>
<html lang=\"ja\">
    <meta charset=\"utf-8\">
    <title>SignUp</title>
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
            }
        </style>

        <script>
            function signup() {
                let object = document.getElementById(\"text_box\");
                let path = location.href;
                if(path.substring(path.length-1) == \"/\") {
                    path.pop();
                }
                location.href = path + \"?\" + object.value;
            }
        </script>
    </head>

    <body>
        <h1>SignUp</h1>
        <input id=\"text_box\" type=\"text\" placeholder=\"NickName\"></input>
        <button onclick=\"signup()\">SigUp</button>
        <div class=\"sign\">built by <a class=\"sign\" href=\"https://github.com/kntt32/\">kntt32</a></div>
    </body>
</html>
";

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

    fn signup_user(&mut self, name: &str, userid: u64) -> usize {
        self.users.push(User { name: name.to_string(), id: userid, score: 0 });
        self.users.len() - 1
    }

    fn get_index_by_id(&self, userid: u64) -> Option<usize> {
        for i in 0 .. self.users.len() {
            if self.users[i].id == userid {
                return Some(i)
            }
        }

        None
    }

    fn is_used_name(&self, name: &str) -> bool {
        if name == "" { return true; }

        for i in 0 .. self.users.len() {
            if self.users[i].name == name {
                return true;
            }
        }
        false
    }

    fn build_userpage(&self, id: u64) -> Result<String, String> {
        if let Some(index) = self.get_index_by_id(id) {
            Ok(
"
<!DOCTYPE html>
<html lang=\"ja\">
    <meta charset=\"utf-8\">
    <title>UserPage</title>
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
            }
        </style>

        <script>
            
        </script>
    </head>

    <body>
        <h1>UserPage</h1>
        <h2>".to_string() + &self.users[index].name + "さん</h2>

        <a href=\"/ranking\">ランキングを見る</a>
        <div class=\"sign\">built by <a class=\"sign\" href=\"https://github.com/kntt32/\">kntt32</a></div>
    </body>
</html>")
        }else {
            Err(ERROR_PAGE_HTML.to_string())
        }
    }

    fn build_ranking(&self) -> Result<String, String> {
        Ok("
<!DOCTYPE html>
<html lang=\"ja\">
    <meta charset=\"utf-8\">
    <title>Ranking</title>
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
            }
        </style>

        <script>
            function signup() {
                let object = document.getElementById(\"text_box\");
                let path = location.href;
                if(path.substring(path.length-1) == \"/\") {
                    path.pop();
                }
                location.href = path + \"?\" + object.value;
            }
        </script>
    </head>

    <body>
        <h1>Ranking</h1>
        
    </body>
</html>
".to_string())
    }
}

impl Service for UsersData {
    fn response(&mut self, url: &str, query: &str) -> Result<String, String> {
        let url_string = url.to_string();
        let url_vec: Vec<&str> = url_string.split("/").collect();

        if 2 <= url_vec.len() && url_vec[1] == "user" {
            if let Ok(userid) = url_vec[2].parse::<u64>() {
                let index_optional = self.get_index_by_id(userid);

                match index_optional {
                    Some(_) => {
                        self.build_userpage(userid)
                    },
                    None => {
                        if !self.is_used_name(query) {
                            self.signup_user(query, userid);
                            println!("new user: {} (@{})", query, userid);
                            self.build_userpage(userid)
                        }else {
                            Ok(SIGNUP_PAGE_HTML.to_string())
                        }

                    }
                }
            }else {
                Err(ERROR_PAGE_HTML.to_string())
            }
        }else if url == "" {
            self.build_ranking()
        }else {
            Err(ERROR_PAGE_HTML.to_string())
        }
    }

    fn save(&self) {

    }
}