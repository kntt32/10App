use server::Service;
use std::fs;
use std::str::from_utf8;


const ADMIN_PASSWORD: &str = "shuma240";
const USERSDATA_FILENAME: &str = "users_data";

static ERROR_PAGE_HTML: &str = "
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
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
                color: black;
            }
        </style>
    </head>

    <body>
        <h1>NotFound</h1>
    </body>
</html>
";

static JUMP_TO_RANKING: &str = "
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
    <title>Redirect</title>
    <head>
        <script>
            window.onload = function() {
                location.href = '/';
            };
        </script>
    </head>
</html>
";

static ADMIN_MODE_AUTH: &str = "
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
                let path = location.href;
                let splitted_path = path.split('?');
                location.href = splitted_path[0] + '?' + object.value;
            }
        </script>
    </head>

    <body>
        <h1>AdminMode</h1>
        <input id='password_box' type='password' placeholder='Password'></input>
        <button onclick='send_password()'>Login</button>
        <div class='sign'>built by <a class='sign' href='https://github.com/kntt32/'>kntt32</a></div>
    </body>
</html>
";


pub struct UsersData {
    users: Vec<User>,
    used_max_userid: u64
}

struct User {
    name: String,
    id: u64,
    score: i32
}

impl UsersData {
    pub fn new() -> UsersData {
        if let Ok(file_bin) = fs::read(USERSDATA_FILENAME) {
            if let Ok(file_string) = from_utf8(&file_bin) {
                let file_vec: Vec<&str> = file_string.split(",").collect();

                let mut users_vec = Vec::new();
                for i in 0 .. file_vec.len()/3 {
                    let id = if let Ok(parsed_id) = file_vec[i*3+1].parse::<u64>() { parsed_id } else { 
                        return UsersData { users: Vec::new(), used_max_userid: 0 }
                    };

                    let score = if let Ok(parsed_score) = file_vec[i*3+2].parse::<i32>() { parsed_score } else { 
                        return UsersData { users: Vec::new(), used_max_userid: 0 }
                    };

                    users_vec.push(User{ name: file_vec[i*3].to_string(), id: id, score: score });
                }
                UsersData { users: users_vec, used_max_userid: 0 }
            }else {
                println!("invalid file");
                UsersData { users: Vec::new(), used_max_userid: 0 } 
            }
        }else {
            println!("file not found");
            UsersData { users: Vec::new(), used_max_userid: 0 } 
        }
    }

    fn reserve_userid(&mut self, n: u64) -> u64 {
        let result = self.used_max_userid;
        self.used_max_userid += n*10000;
        result
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
        if name.to_string().contains(",") { return true; }

        for i in 0 .. self.users.len() {
            if self.users[i].name == name {
                return true;
            }
        }
        false
    }

    fn set_score(&mut self, userid: u64, score: i32) {
        if let Some(index) = self.get_index_by_id(userid) {
            self.users[index].score = score;
        }
        self.make_ranking();
    }

    fn make_ranking(&mut self) {
        let compare = | x: &User , y: &User | { y.score.cmp(&x.score) };
        self.users.sort_by(compare);
    }

    fn build_signuppage(msg: &str) -> String {
        let insert_message = if msg.len() != 0 {
            "<div>".to_string() + msg + "</div>"
        }else {
            String::new()
        };

        "
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
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
                color: black;
            }
        </style>

        <script>
            function signup() {
                let object = document.getElementById('text_box');
                let path = location.href;
                if(path.substring(path.length-1) == '/') {
                    path = path.substring(0, path.length-1);
                }
                let encoded_user_name = encodeURI(object.value);
                location.href = path.split('?')[0] + '?' + encoded_user_name;
            }
        </script>
    </head>

    <body>
        <h1>SignUp</h1>
".to_string()
 + &insert_message
+ "
        <input id='text_box' type='text' placeholder='NickName'></input>
        <button onclick='signup()'>SignUp</button>
        <div class='sign'>built by <a class='sign' href='https://github.com/kntt32/'>kntt32</a></div>
    </body>
</html>
"
    }

    fn build_userpage(&self, id: u64) -> Result<String, String> {
        if let Some(index) = self.get_index_by_id(id) {
            Ok(
"
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
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
                color: black;
            }

            #score_board {
                width: 300px;
                height: 120px;
                font-size: 60px;
                border-radius: 0px;
                border-width: 0px;
                background: #dee9ec;
                margin: 10px;

                display: grid;
                justify-items: center;
                align-content: center;
            }

            #link_to_ranking {
                color: #929292;
                margin: 20px;
            }

            #admin_mode_button {
                position: absolute;
                top: 5px;
                left: 5px;

                display: grid;
                justify-items: center;
                align-content: center;

                width: 40px;
                height: 40px;
            }

            .circle {
                background: #929292;
                border-radius: 5px;
                width: 5px;
                height: 5px;
                border-width: 0px;
                margin: 0px;
                margin-top: 5px;
                margin-bottom: 5px;
            }
        </style>

        <script>
            function admin_mode() {
                let path = location.href;
                if(path.substring(path.length - 1) == '/') {
                    path = path.substring(0, path.length-1);
                }
                location.href = path.split('?')[0] + '/admin'
            }

            window.onload = function() {
                document.getElementById('user_name').textContent = decodeURI('".to_string() + &self.users[index].name +"');
            };
        </script>
    </head>

    <body>
        <h1>UserPage</h1>
        <h2 id='user_name'>-</h2>

        <div id='score_board'>
        " + &self.users[index].score.to_string() +"
        </div>

        <a id='link_to_ranking' href='/ranking'>ランキングを見る</a>
        <div class='sign'>built by <a class='sign' href='https://github.com/kntt32/'>kntt32</a></div>

        <button id='admin_mode_button' onclick='admin_mode();'>
            <div>
                <div class='circle'></div>
                <div class='circle'></div>
                <div class='circle'></div>
            </div>
        </button>
    </body>
</html>")
        }else {
            Err(ERROR_PAGE_HTML.to_string())
        }
    }

    fn build_adminmode(&self, userid: u64, msg: &str) -> Result<String, String> {
        let index_optional = self.get_index_by_id(userid);

        let mut score: i32 = 0;
        if let Some(index) = index_optional {
            score = self.users[index].score;
        }

        Ok("
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
            function submit() {
                let path = location.href;
                if(path.substring(path.length - 1) == '/') {
                    path = path.substring(0, path.length-1);
                }
                let splitted_path = path.split('?');
                let query = '';
                if(2 <= splitted_path.length) {
                    query = splitted_path[1];
                }

                let score = parseInt(document.getElementById('score_textbox').value) + ".to_string() + &score.to_string() + ";
                location.href = path.split('?')[0] + '/' + score + '?' + query;
            }
        </script>
    </head>

    <body>
        <h1>AdminMode</h1>
        <label for='score_textbox'>スコア" + msg + "</label>
        <input id='score_textbox' type='number' value='10' placeholder='スコアを入力'></input>
        <button onclick='submit()'>Submit</button>
        <div class='sign'>built by <a class='sign' href='https://github.com/kntt32/'>kntt32</a></div>
    </body>
</html>
")
    }

    fn build_ranking(&self) -> Result<String, String> {
        let mut ranking_html = String::new();

        if self.users.len() == 0 {
            ranking_html = String::from("No User");
        }else {
            for i in 0 .. self.users.len() {
                if i == 0 {
                    ranking_html.push_str(&("
                        <div style='display: flex'>
                        <div class='ranking_first' style='float: left;width: 40px;'>".to_string()+ &(i+1).to_string() +"</div>
                        <div class='ranking_first' style='float: left;width: 160px;'>"+ &self.users[i].name +"</div>
                        <div class='ranking_first' style='float: left;width: 100px;'>"+ &self.users[i].score.to_string() +"</div>
                        </div>
                        "));
                }else if i <= 2 {
                    ranking_html.push_str(&("
                        <div style='display: flex'>
                        <div class='ranking_best3' style='float: left;width: 40px;'>".to_string()+ &(i+1).to_string() +"</div>
                        <div class='ranking_best3' style='float: left;width: 160px;'>"+ &self.users[i].name +"</div>
                        <div class='ranking_best3' style='float: left;width: 100px;'>"+ &self.users[i].score.to_string() +"</div>
                        </div>
                        "));
                }else {
                    ranking_html.push_str(&("
                        <div style='display: flex'>
                        <div class='ranking' style='float: left;width: 40px;'>".to_string()+ &(i+1).to_string() +"</div>
                        <div class='ranking' style='float: left;width: 160px;'>"+ &self.users[i].name +"</div>
                        <div class='ranking' style='float: left;width: 100px;'>"+ &self.users[i].score.to_string() +"</div>
                        </div>
                        "));
                }
            }
        }

        Ok("
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
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
                color: black;
            }
            
            .ranking_first {
                font-size: 25px;
                height: 60px;
                background: #ffd977;
                margin: 5px;

                display: grid;
                justify-items: center;
                align-content: center;

                border-width: 0px;
            }

            .ranking_best3 {
                font-size: 25px;
                height: 60px;
                background: #9ac9d9;
                margin: 5px;

                display: grid;
                justify-items: center;
                align-content: center;

                border-width: 0px;
            }

            .ranking {
                font-size: 25px;
                height: 60px;
                background: #dee8ed;
                margin: 5px;

                display: grid;
                justify-items: center;
                align-content: center;

                border-width: 0px;
            }
        </style>

        <script>

        setTimeout(function () {
            location.reload();
        }, 60000);
        </script>
    </head>

    <body>
        <h1>Ranking</h1>
        ".to_string() + 
&ranking_html
         + "
    </body>
</html>
")
    }
}

impl Service for UsersData {
    fn response(&mut self, url: &str, query: &str) -> Result<String, String> {
        let url_string = url.to_string();
        let url_vec: Vec<&str> = url_string.split("/").collect();

        if 3 <= url_vec.len() && url_vec[1] == "user" {
            if let Ok(userid) = url_vec[2].parse::<u64>() {
                let index_optional = self.get_index_by_id(userid);

                match index_optional {
                    Some(_) => {
                        if url_vec.len() == 3 {
                            self.build_userpage(userid)
                        }else if url_vec[3] == "admin" {
                            match url_vec.len() {
                                4 => if query == ADMIN_PASSWORD {
                                        self.build_adminmode(userid, "")
                                    }else {
                                        Ok(ADMIN_MODE_AUTH.to_string())
                                    },
                                5 => if query == ADMIN_PASSWORD {
                                        if let Ok(score) = url_vec[4].parse::<i32>() {
                                            self.set_score(userid, score);
                                            Ok(JUMP_TO_RANKING.to_string())
                                        }else {
                                            Err(ERROR_PAGE_HTML.to_string())
                                        }
                                    }else {
                                        Err(ERROR_PAGE_HTML.to_string())
                                    }
                                _ => Err(ERROR_PAGE_HTML.to_string())
                            }
                        }else {
                            Err(ERROR_PAGE_HTML.to_string())
                        }
                    },
                    None => {
                        if !self.is_used_name(query) {
                            self.signup_user(query, userid);
                            println!("new user: {} (@{})", query, userid);
                            self.build_userpage(userid)
                        }else {
                            if query.len() == 0 {
                                Ok(UsersData::build_signuppage(""))
                            }else {
                                Ok(UsersData::build_signuppage("使用できない名前です"))
                            }
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
        let mut file_text = String::new();

        for i in 0 .. self.users.len() {
            file_text.push_str(&self.users[i].name);
            file_text.push(',');
            file_text.push_str(&self.users[i].id.to_string());
            file_text.push(',');
            file_text.push_str(&self.users[i].score.to_string());
            file_text.push(',');
        }

        fs::write(USERSDATA_FILENAME, file_text.as_bytes()).expect("Err: Fail to Save");
    }

    fn reset(&mut self) {
        self.users = Vec::new();

        fs::write(USERSDATA_FILENAME, b"").expect("Err: Fail to Save");
    }

    fn service_admin(&mut self) -> Result<String, String> {

        Ok("
<!DOCTYPE html>
<html lang='ja'>
    <meta charset='utf-8'>
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
                color: black;
            }
        </style>
    </head>

    <body>
        <h1>CreateQRCode</h1>
    </body>
</html>
".to_string())

    }
}