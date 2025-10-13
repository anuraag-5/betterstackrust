use crate::{store::Store};
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub user_id: String,
    pub time_added: NaiveDateTime,
    pub is_snippet_added: bool,
    pub about: String
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
    pub fn create_website(&mut self, u_i: String, new_url: String, input_about: String) -> Result<Website, Error> {
        let new_website = Website {
            id: Uuid::new_v4().to_string(),
            url: new_url,
            time_added: Utc::now().naive_local(),
            user_id: u_i,
            is_snippet_added: false,
            about: input_about
        };

        let created_website = diesel::insert_into(crate::schema::websites::table)
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
        use crate::schema::websites::dsl::*;

        let website_result = websites
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
        use crate::schema::websites::dsl::*;

        let found_website = websites
            .filter(url.eq(input_url))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        Ok(found_website)
    }

    pub fn get_all_websites(&mut self) -> Result<Vec<(String, String, String, bool)>, diesel::result::Error> {
        use crate::schema::websites::dsl::*;
    
        let websites_result = websites
            .select((url, id, user_id, is_snippet_added))
            .load::<(String, String, String, bool)>(&mut self.conn)?;
    
        Ok(websites_result)
    }

    pub fn get_users_all_websites(&mut self, input_user_id: String) -> Result<Vec<Website>, Error> {
        use crate::schema::websites::dsl::*;

        let websites_result = websites.filter(user_id.eq(input_user_id)).select(Website::as_select()).load(&mut self.conn)?;

        Ok(websites_result)
    }
}
