use crate::store::Store;
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: NaiveDateTime,
}

impl Store {
    pub fn create_website(&mut self, u_i: String, new_url: String) -> Result<Website, Error> {
        let new_website = Website {
            id: Uuid::new_v4().to_string(),
            url: new_url,
            time_added: Utc::now().naive_local(),
            user_id: u_i,
        };

        let created_website = diesel::insert_into(crate::schema::website::table)
            .values(new_website)
            .returning(Website::as_returning())
            .get_result(&mut self.conn);

        match created_website {
            Ok(w) => Ok(w),
            Err(e) => Err(e),
        }
    }

    pub fn get_website(&mut self, input_website_id: String, input_user_id: String) -> Result<Website, Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
            .filter(id.eq(input_website_id))
            .filter(user_id.eq(input_user_id))
            .select(Website::as_select())
            .first(&mut self.conn);

        match website_result {
            Ok(w) => Ok(w),
            Err(e) => Err(e),
        }
    }
}
