use domain::models::{Todo, NewTodo};
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_todo(todo: NewTodo) -> Created<String> {
    use domain::schema::todos;

    match diesel::insert_into(todos::table).values(&todo).get_result::<Todo>(&mut establish_connection()) {
        Ok(todo) => {
            let response = Response { body: ResponseBody::Todo(todo) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}