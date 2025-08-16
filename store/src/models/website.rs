use crate::store::Store;
use diesel::{prelude::*};

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Website {
    id: String,
    url: String,
    user_id: String,
}

impl Store {
    pub fn create_website(&self) {
        print!("create user called")
    }
    pub fn get_website(&self) -> String {
        String::from("1")
    }
}