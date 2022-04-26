use rocket_dyn_templates::{Template};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct IndexContext<'a>{
    foo: u64,
    bar: &'a str,
}

// For passing Empty Context
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EmptyContext{}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Task{
    cost: u32,
    reward: u32
}

// Context Example
#[get("/<foo>")]
pub fn hello(foo: u64) -> Template {

    Template::render("tera/foo", IndexContext{
        foo,
        bar: "Hello Word",
    })
}

// JSON Example 
#[get("/task/<cost>/<reward>")]
pub fn todo(cost: u32, reward: u32) -> Json<Task> {
    Json(Task {cost: cost, reward: reward} )

}

#[post("/task", format="json", data="<task_input>")]
pub fn post_todo(task_input: Json<Task>) -> Json<Task> {
    let str: String = format!("Json Response: cost: {}\n reward: {}", task_input.cost, task_input.reward);
    println!("{}", str); 
    task_input 
}

// Default View
#[get("/")]
pub fn tera_index() -> Template {
    Template::render("tera/index", EmptyContext {})

}