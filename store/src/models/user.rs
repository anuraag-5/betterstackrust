use crate::store::Store;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
}

pub struct UserOutput {
    pub id: String,
    pub email: String,
    pub name: String,
}

impl Store {
    pub fn sign_up(
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
        };

        let result = diesel::insert_into(crate::schema::users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn);

        match result {
            Ok(u) => {
                return Ok(u.id);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn sign_in(
        &mut self,
        input_email: String,
        user_password: String,
    ) -> Result<UserOutput, Error> {
        use crate::schema::users::dsl::*;

        let signed_in_user = users
            .filter(email.eq(input_email))
            .select(User::as_select())
            .load(&mut self.conn);

        match signed_in_user {
            Ok(u) => {
                if u.len() > 0 && u[0].password == user_password {
                    Ok(UserOutput {
                        id: u[0].id.clone(),
                        email: u[0].email.clone(),
                        name: u[0].name.clone(),
                    })
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }

            Err(_) => Err(diesel::result::Error::NotFound),
        }
    }
}
