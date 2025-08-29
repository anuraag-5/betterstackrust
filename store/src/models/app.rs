use crate::{schema::page_visits, store::Store};
use diesel::{prelude::*, result::Error};

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::page_visits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PageVisit {
    pub visitor_id: String,
    pub page_url: String,
    pub referrer: String,
    pub user_agent: String,
    pub website_id: String,
}
impl Store {
    pub fn store_tracks(&mut self, page_visit_data: PageVisit) -> Result<PageVisit, Error> {
        let created_page_visit = diesel::insert_into(page_visits::table)
            .values(page_visit_data)
            .returning(PageVisit::as_returning())
            .get_result(&mut self.conn)?;

        Ok(created_page_visit)
    }
}
