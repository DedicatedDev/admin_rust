#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

#[cfg(test)] mod tests;

use std::collections::HashMap;

use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[derive(Serialize)]
struct TemplateContext {
    items: Vec<&'static str>
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("index", &context)
}

#[get("/buttons")]
fn buttons() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("buttons", &context)
}

#[get("/cards")]
fn cards() -> Template {
    let context = TemplateContext { items: vec!["SBAdmin2 Rust by Isaac"]};
    Template::render("cards", &context)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, buttons, cards])
        .mount("/", StaticFiles::from("templates"))
        .attach(Template::fairing())
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}