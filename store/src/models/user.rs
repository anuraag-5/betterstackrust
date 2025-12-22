use crate::store::Store;
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub plan_name: String
}

pub struct UserOutput {
    pub id: String,
    pub email: String,
    pub name: String,
    pub plan_type: String
}

impl Store {
    pub async fn sign_up(
        &mut self,
        username: String,
        user_password: String,
        user_name: String,
    ) -> Result<String, Error> {
        let new_user = User {
            id: Uuid::new_v4().to_string(),
            email: username,
            password: user_password,
            name: user_name,
            plan_name: "Basic".to_string()
        };

        let result = diesel::insert_into(crate::schema::users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn).await;

        match result {
            Ok(u) => {
                return Ok(u.id);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn sign_in(
        &mut self,
        input_email: String,
        user_password: String,
    ) -> Result<UserOutput, Error> {
        use crate::schema::users::dsl::*;

        let signed_in_user = users
            .filter(email.eq(input_email))
            .select(User::as_select())
            .load(&mut self.conn).await;

        match signed_in_user {
            Ok(u) => {
                if u.len() > 0 && u[0].password == user_password {
                    Ok(UserOutput {
                        id: u[0].id.clone(),
                        email: u[0].email.clone(),
                        name: u[0].name.clone(),
                        plan_type: u[0].plan_name.clone()
                    })
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }

            Err(_) => Err(diesel::result::Error::NotFound),
        }
    }

    pub async fn update_email(&mut self,
        input_user_id: String,
        new_email: String
    ) -> Result<usize, Error> {
        let query = r#"
        Update users set email = $1 where id = $2;
        "#;

        let res = diesel::sql_query(query).bind::<diesel::sql_types::Text, _>(new_email).bind::<diesel::sql_types::Text, _> (input_user_id).execute(&mut self.conn).await?;

        Ok(res)
    }
    
    pub async fn update_password(&mut self,
        input_user_id: String,
        old_password: String,
        new_password: String
    ) -> Result<usize, Error> {
        let query = r#"
        Update users set password = $1 where id = $2 AND password = $3;
        "#;

        let res = diesel::sql_query(query).bind::<diesel::sql_types::Text, _>(new_password).bind::<diesel::sql_types::Text, _> (input_user_id)
        .bind::<diesel::sql_types::Text, _>(old_password).execute(&mut self.conn).await?;

        Ok(res)
    }
}
