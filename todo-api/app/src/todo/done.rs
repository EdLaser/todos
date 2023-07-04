use domain::models::Todo;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use diesel::prelude::*;

pub fn done_todo(todo_id: i32) -> Result<Todo, NotFound<String>> {
    use domain::schema::todos::dsl::*;

    match diesel::update(todos.find(todo_id)).set(done.eq(true)).get_result::<Todo>(&mut establish_connection()) {
        Ok(todo) => Ok(todo),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error marking todo with id {} as done - {}", todo_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}