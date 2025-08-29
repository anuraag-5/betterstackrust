use crate::{store::Store};
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

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::website_tick)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WebsiteTick {
    pub id: String,
    pub response_time_ms: i32,
    pub status: String,
    pub region_id: String,
    pub website_id: String,
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

    pub fn get_website(
        &mut self,
        input_website_id: String,
        input_user_id: String,
    ) -> Result<Website, Error> {
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

    pub fn search_website(&mut self, input_url: &str) -> Result<Website, Error> {
        use crate::schema::website::dsl::*;

        let found_website = website
            .filter(url.eq(input_url))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(found_website)
    }

    pub fn get_all_websites(&mut self) -> Result<Vec<(String, String, String)>, diesel::result::Error> {
        use crate::schema::website::dsl::*;
    
        let websites = website
            .select((url, id, user_id))
            .load::<(String, String, String)>(&mut self.conn)?;
    
        Ok(websites)
    }

}
