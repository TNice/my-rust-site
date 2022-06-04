#[macro_use] extern crate rocket;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::form::Form;
use rocket::fs::{FileServer, relative};
use rocket::http::{Status, CookieJar, Cookie};
use rocket_dyn_templates::{Template};
use rocket::serde::{Deserialize, Serialize, json::Json};
use std::borrow::Borrow;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::collections::HashMap;
use std::fs::OpenOptions;
use crypto::digest::Digest;
use crypto::sha3::Sha3;

// ------ Structs ------
#[derive(serde::Serialize)]
struct IndexContext {
    user_id: String
}

#[derive(serde::Serialize)]
struct LoginContext {
    user_id: String,
    message: String
}

#[derive(serde::Serialize)]
struct AboutContext{
    parent: &'static str,
}

#[derive(serde::Serialize)]
struct ProjectContext{
    user_id: String,
    paths: Vec<String>,
    file_names: Vec<String>
}

#[derive(Deserialize)]
pub struct AuthInfo {
    pub user_id: String,
    pub password_hash: String,
}

#[derive(FromForm, Deserialize, Serialize)]
struct User {
    name: String,
    password: String
}

#[derive(FromForm, Deserialize)]
struct CreateInfo{
    username: String,
    password: String
}

#[derive(FromForm, Deserialize)]
struct LoginInfo{
    username: String,
    password: String
}

// ------ Functions ------
fn hash_pass(password: &String) -> String{
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

fn find_user(user: String) -> bool {
    let input = File::open("C:/Users/tyler/OneDrive/Desktop/RustStuff/our-site/site-server/src/users.txt");
    let buffer = BufReader::new(input.unwrap());

    for line in buffer.lines(){
        let l: String = line.unwrap();
        let parts: Vec<&str> = l.split(":").collect();

        if parts[0].eq(&user){
            return true
        }
    }
    false
}

// ------ Users "/users" Handlers ------
#[post("/create", data="<create_info>")]
fn create(create_info: Form<CreateInfo>) -> Result<Redirect, Flash<Redirect>> {
    let pass_hash = hash_pass(&create_info.password);
    
    if find_user(create_info.username.clone()) == true{
        return Err(Flash::error(Redirect::to(uri!(register)), "Username already exists"))
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("C:/Users/tyler/OneDrive/Desktop/RustStuff/our-site/site-server/src/users.txt")
        .unwrap();

    if let Err(_e) = writeln!(file, "{}:{}", create_info.username, pass_hash){
        return Err(Flash::error(Redirect::to(uri!(register)), "Error Creating User"))
    }
    else{
        Ok(Redirect::to(uri!(login_page)))
    }
}

#[post("/login", data="<login_info>")]
fn login(login_info: Form<LoginInfo>, cookies: &CookieJar<'_>) -> Result<Redirect, Flash<Redirect>> {
    let input = File::open("C:/Users/tyler/OneDrive/Desktop/RustStuff/our-site/site-server/src/users.txt");
    let buffer = BufReader::new(input.unwrap());

    for line in buffer.lines(){
        let l: String = line.unwrap();
        let parts: Vec<&str> = l.split(":").collect();

        if parts[0].eq(&login_info.username){
            let hash = hash_pass(&login_info.password);
            if parts[1] == hash{
                cookies.add_private(Cookie::new("user_id", login_info.username.clone()));    
            }
            else{
                return Err(Flash::error(Redirect::to(uri!(login)), "Username or Password Incorrect"))
            }
            break;
        }      
    }
    Ok(Redirect::to(uri!(index)))
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect{
    cookies.remove_private(Cookie::named("user_id"));

    Redirect::to(uri!(index))
}


// ------ Root "/" Handlers ------
#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
    let cookie = cookies.get_private("user_id");
    let mut user_id = "none".to_string();
    
    if cookie.is_none() == false{
        user_id = cookie.unwrap().value().to_string();
    }

    Template::render("index", &IndexContext{
        user_id: user_id
    })
}

#[get("/login")]
fn login_page(flash: Option<FlashMessage<'_>>, cookies: &CookieJar<'_>) -> Result<Template, Redirect>{
    let cookie = cookies.get_private("user_id");
    if cookie.is_none() == false{
        return Err(Redirect::to(uri!(index)));
    }

    let msg = flash.map(|flash| format!("{}", flash.message()))
        .unwrap_or_else(|| "none".to_string());

    return Ok(Template::render("login", &LoginContext{
        user_id: "none".to_string(),
        message: msg
    }));
}

#[get("/register")]
fn register(flash: Option<FlashMessage<'_>>, cookies: &CookieJar<'_>) -> Result<Template, Redirect>{
    let cookie = cookies.get_private("user_id");
    if cookie.is_none() == false{
        return Err(Redirect::to(uri!(index)));
    }

    let msg = flash.map(|flash| format!("{}", flash.message()))
        .unwrap_or_else(|| "none".to_string());

    return Ok(Template::render("register", &LoginContext{
        user_id: "none".to_string(),
        message: msg
    }));
}

#[get("/about")]
fn about(cookies: &CookieJar<'_>) -> Template {
    let cookie = cookies.get_private("user_id");
    let mut user_id = "none".to_string();
    
    if cookie.is_none() == false{
        user_id = cookie.unwrap().value().to_string();
    }

    Template::render("about", &IndexContext{
        user_id: user_id
    })
}

// ------ Projects "/projects" Handlers ------
impl ProjectContext{
    fn new(paths: Vec<String>, file_names: Vec<String>, user_id: String) -> ProjectContext{ 
        ProjectContext {
            user_id: user_id,
            paths: paths,
            file_names: file_names
        }
    }
}

#[get("/<project_name>")]
fn projects(project_name: String, cookies: &CookieJar<'_>) -> Template{
    
    let cookie = cookies.get_private("user_id");
    let mut user_id = "none".to_string();
    
    if cookie.is_none() == false{
        user_id = cookie.unwrap().value().to_string();
    }
    
    let projects: HashMap<String, ProjectContext> = HashMap::from([
        ("rust-site".to_string(), ProjectContext::new(vec!["/static/rust-site/main.rs".to_string()], 
            vec!["src/main.rs".to_string()], 
            user_id
        ))
    ]);

    Template::render("project_site", projects.get(&project_name).unwrap())
}



// ------ Launch Site ------
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/static", FileServer::from(relative!("/static")))
    .mount("/", routes![index, about, login_page, register])
    .mount("/users", routes![logout, login, create])
    .mount("/projects", routes![projects])
}