use app::todo::{create, delete, done, read};
use domain::models::{NewTodo, Todo};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, response::status::Created, response::status::NotFound};
use shared::response_models::{Response, ResponseBody};

#[get("/")]
pub fn list_todos_handler() -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let todos: Vec<Todo> = read::list_todos();
    let response = Response {
        body: ResponseBody::Todos(todos),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/todo/<todo_id>")]
pub fn list_todo_handler(todo_id: i32) -> Result<String, NotFound<String>> {
    // 👇 New function body!
    let todo = read::list_todo(todo_id)?;
    let response = Response {
        body: ResponseBody::Todo(todo),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[put("/<todo_id>")]
pub fn done_todo_handler(todo_id: i32) -> Result<String, NotFound<String>> {
    let todo = done::done_todo(todo_id)?;
    let response = Response {
        body: ResponseBody::Todo(todo),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_todo", format = "application/json", data = "<todo>")]
pub fn create_todo_handler(todo: Json<NewTodo>) -> Created<String> {
    let todo = todo.into_inner();
    create::create_todo(todo)
}

/* #[post("/new_todos", format = "application/json", data = "<todos>")]
pub fn create_todos_handler(todos: Json<Vec<NewTodo>>) -> Created<String> {

    for todo in todos.iter() {
        create::create_todo(todo);
    }
} */

#[delete("/delete/<todo_id>")]
pub fn delete_todo_handler(todo_id: i32) -> Result<String, NotFound<String>> {
    let todos = delete::delete_todo(todo_id)?;
    let response = Response {
        body: ResponseBody::Todos(todos),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_todos", format = "application/json", data = "<todos>")]
pub fn new_todos_handler(todos: Json<Vec<NewTodo>>) -> Created<String> {
    for todo in todos.into_inner() {
        create::create_todo(todo);
    }
    Created::new("/new_todos".to_string())
}
