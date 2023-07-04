use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;
use domain::models::Todo;

pub fn delete_todo(todo_id: i32) -> Result<Vec<Todo>, NotFound<String>> {
    use domain::schema::todos::dsl::*;
    use domain::schema::todos;

    let response: Response;

    let num_deleted = match diesel::delete(todos.filter(id.eq(todo_id))).execute(&mut establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error deleting post with id {} - {}", todo_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        match todos::table.select(todos::all_columns).load::<Todo>(&mut establish_connection()) {
            Ok(mut todos_) => {
                todos_.sort();
                Ok(todos_)
            },
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no post with id {}", todo_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}