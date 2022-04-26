#[macro_use] extern crate rocket;
mod tera;

use rocket_dyn_templates::Template;
use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;

use rocket::form::{Form, FromForm, FromFormField};
use rocket::http::RawStr;


#[derive(FromForm)]
struct UserInput<'f> {
    value: &'f str
}

#[post("/submit", data = "<user_input>")]
fn submit_task(user_input: Form<UserInput>) -> String {
    // format!("Your Value: {}", user_input.value)
    println!("recieved!");
    format!("Youre value, {}", user_input.value)
}

#[get("/")]
fn index() -> Redirect {
    Redirect::temporary("/tera")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/form", routes![submit_task])
        .mount("/tera", routes![tera::hello, 
                                tera::todo,     
                                tera::tera_index,
                                tera::post_todo
                                ])
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("/static")))
       
}