use rocket::{get, post, response::status::NotFound,response::status::Created};
use shared::response_models::{Response, ResponseBody};
use app::todo::{create, read, delete, done};
use domain::models::{Todo, NewTodo};
use rocket::serde::json::Json;

#[get("/")]
pub fn list_todos_handler() -> String {
    // 👇 New function body!
    let todos: Vec<Todo> = read::list_todos();
    let response = Response { body: ResponseBody::Todos(todos) };

    serde_json::to_string(&response).unwrap()
}

#[get("/todo/<todo_id>")]
pub fn list_todo_handler(todo_id: i32) -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let todo = read::list_todo(todo_id)?;
    let response = Response { body: ResponseBody::Todo(todo) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/done/<todo_id>")]
pub fn done_todo_handler(todo_id: i32) -> Result<String, NotFound<String>> {
    let todo = done::done_todo(todo_id)?; 
    let response = Response { body: ResponseBody::Todo(todo) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_todo", format = "application/json", data = "<todo>")]
pub fn create_todo_handler(todo: Json<NewTodo>) -> Created<String> {
    create::create_todo(todo)
}

#[get("/delete/<todo_id>")]
pub fn delete_todo_handler(todo_id: i32) -> Result<String, NotFound<String>> {
    let todos = delete::delete_todo(todo_id)?;
    let response = Response { body: ResponseBody::Todos(todos) };

    Ok(serde_json::to_string(&response).unwrap())
}