use crate::schema::todos;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Selectable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub done: bool,
}


#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: &'a str,
}