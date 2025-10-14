use crate::{models::user::{User, UserOutput}, schema::page_visits, store::Store};
use diesel::{dsl::count, prelude::*, result::Error};

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::page_visits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PageVisit {
    pub visitor_id: String,
    pub page_url: String,
    pub referrer: String,
    pub user_agent: String,
    pub website: String,
}

impl Store {

    pub fn get_user(&mut self, input_user_id: String) -> Option<UserOutput> {
        use crate::schema::users::dsl::*;

        let user = users.filter(id.eq(input_user_id)).select(User::as_select()).get_result(&mut self.conn);

        match user {
            Ok(user) => {
                Some(UserOutput { id: user.id.clone(), email: user.email.clone(), name: user.name.clone() })
            },
            Err(_) => {
                println!("User not found");
                None
            }
        }
    }

    pub fn store_tracks(&mut self, page_visit_data: PageVisit) -> Result<PageVisit, Error> {
        let created_page_visit = diesel::insert_into(page_visits::table)
            .values(page_visit_data)
            .returning(PageVisit::as_returning())
            .get_result(&mut self.conn)?;

        Ok(created_page_visit)
    }

    pub fn get_total_views(&mut self, input_website_url: String) -> Result<i64, Error>{
        use crate::schema::page_visits::dsl::*;

        let total_visitors = page_visits.filter(website.eq(input_website_url)).select(count(website)).get_result::<i64>(&mut self.conn)?;

        Ok(total_visitors)
    }
}
