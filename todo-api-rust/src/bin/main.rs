
#[macro_use] extern crate rocket;
use api::todo_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
            .mount("/api", routes![
                todo_handler::list_todos_handler,
                todo_handler::list_todo_handler,
                todo_handler::create_todo_handler,
                todo_handler::delete_todo_handler,
                todo_handler::done_todo_handler
            ])
}