#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate bcrypt;
extern crate rusqlite;

extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket::http;
use rocket::Response;
use rocket::response::{Redirect, NamedFile, Failure};
use rocket::request::{Form, Request};
use rocket_contrib::Template;

use std::path::{PathBuf, Path};

mod contexts;
mod forms;
mod db;

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/home")
}

#[get("/home")]
fn home(cookies: &http::Cookies) -> Redirect {
    match cookies.find("user") {
        Some(cookie) => Redirect::to(
            format!("/home/{}", cookie.value()).as_str()),
        None => Redirect::to("/login"),
    }
}

#[get("/home/<user>", rank = 2)]
fn home_user(user: String) -> Result<Template, Failure> {
    let db = match db::get() {
        Ok(connection) => connection,
        Err(_) => return Err(Failure(http::Status::InternalServerError)),
    };

    match db.query_row(
        "SELECT * FROM users WHERE name=?1",
        &[&user],
        |row| {row.get::<&str, String>("name")},
        ) {
        Ok(_) => { /* continue */ },
        Err(_) => return Err(Failure(http::Status::NotFound)),
    };

    let context = contexts::User {
        user: user,
        movies: vec![
            "Bee Movie".to_string(),
            "Home Alone".to_string()
        ],
    };

    Ok(Template::render("index", &context))
}

#[get("/login")]
fn login_page() -> Template {
    let context = contexts::Empty {};

    Template::render("login", &context)
}

#[post("/login", data = "<user>")]
fn login(cookies: &http::Cookies, user: Form<forms::Login>)
         -> Result<Redirect, Failure> {
    // get user data from inside the Form object
    let login_data = user.into_inner();
    let name = login_data.name.clone();
    let mut pass = login_data.pass.clone();
    drop(login_data);

    // hash the password
    pass = match bcrypt::hash(pass.as_str(), bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(Failure(http::Status::InternalServerError)),
    };

    // get a db connection
    let db = match db::get() {
        Ok(connection) => connection,
        Err(_) => return Err(Failure(http::Status::InternalServerError)),
    };

    // look up password in db and compare to transmitted password
    let success = match db.query_row::<String, _>(
        "SELECT password FROM users WHERE name=?1",
        &[&name],
        |row| {row.get(0)})
            {
                Ok(p) => match bcrypt::verify(p.as_str(), pass.as_str()) {
                    Ok(valid) => valid,
                    Err(_) => return Err(Failure(http::Status::InternalServerError)),
                },
                // return forbidden if the password is wrong
                Err(_) => return Err(Failure(http::Status::Forbidden)),
            };

    if !success {
        panic!("impossible!");
    }

    // add cookie for username
    cookies.add(http::Cookie::new("user", name));

    // add cookie for session token
    cookies.add(http::Cookie::new("session", "goodsessiontoken"));

    Ok(Redirect::to("/"))
}

//#[post("/register")]
//fn register(form: Form<forms::Register>) -> Redirect {
    //// TODO
    //Redirect::to("/")
//}

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let context = contexts::NotFound {
        uri: req.uri().to_string(),
    };

    Template::render("404", &context)
}

fn main() {
    let routes = routes![
        root,
        home,
        home_user,
        login_page,
        login,
        static_content
    ];

    let errors = errors![
        not_found,
    ];

    rocket::ignite()
        .mount("/", routes)
        .catch(errors)
        .launch();
}
