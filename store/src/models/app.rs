use crate::{models::user::{User, UserOutput}, schema::page_visits, store::Store};
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::page_visits)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PageVisit {
    pub visitor_id: String,
    pub page_path: String,
    pub referrer: String,
    pub user_agent: String,
    pub website: String,
}

impl Store {
    pub async fn get_user(&self, input_user_id: String) -> Result<Option<UserOutput>, Error> {
        use crate::schema::users::dsl::*;
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;
        let user = users.filter(id.eq(input_user_id)).select(User::as_select()).get_result(&mut conn).await;

        match user {
            Ok(user) => {
                Ok(Some(UserOutput { id: user.id, email: user.email, name: user.name, plan_type: user.plan_name }))
            },
            Err(_) => {
                println!("User not found");
                Ok(None)
            }
        }
    }

    pub async fn store_tracks(&self, page_visit_data: PageVisit) -> Result<PageVisit, Error> {
        let mut conn = self.pool.get().await
        .map_err(|e| { println!("{}", e.to_string()); return Error::NotFound })?;
        let created_page_visit = diesel::insert_into(page_visits::table)
            .values(page_visit_data)
            .returning(PageVisit::as_returning())
            .get_result(&mut conn).await?;

        Ok(created_page_visit)
    }
}
