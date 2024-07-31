#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Welcome");
    context.insert("greeting", "Hello, world!");
    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
