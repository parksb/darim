use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::models::db_connection;
use crate::schema::{users, users::dsl};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArgs {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateArgs {
    pub name: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserDAO {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct UserRepository {
    pub conn: MysqlConnection,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            conn: db_connection::connect(),
        }
    }

    pub fn find_by_id(&self, id: u64) -> Result<User, Error> {
        let user: User = dsl::users.find(id).get_result::<User>(&self.conn)?;
        Ok(user)
    }

    pub fn find_by_email(&self, email: &str) -> Result<User, Error> {
        let user: User = dsl::users
            .filter(dsl::email.eq(email))
            .get_result::<User>(&self.conn)?;
        Ok(user)
    }

    pub fn find_password_by_email(&self, email: &str) -> Result<String, Error> {
        let password: String = dsl::users
            .select(dsl::password)
            .filter(dsl::email.eq(email))
            .get_result::<String>(&self.conn)?;
        Ok(password)
    }

    pub fn find_all(&self) -> Result<Vec<User>, Error> {
        let user_list: Vec<User> = dsl::users.load::<User>(&self.conn)?;
        Ok(user_list)
    }

    pub fn create(
        &self,
        name: &str,
        email: &str,
        password: &str,
        avatar_url: &Option<String>,
    ) -> Result<usize, Error> {
        let user_to_create = UserDAO {
            id: None,
            name: Some(name.to_string()),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            avatar_url: avatar_url.clone(),
            updated_at: None,
        };

        let count = diesel::insert_into(dsl::users)
            .values(user_to_create)
            .execute(&self.conn)?;

        Ok(count)
    }

    pub fn update(
        &self,
        id: u64,
        name: &Option<String>,
        password: &Option<String>,
        avatar_url: &Option<String>,
    ) -> Result<usize, Error> {
        let user_to_update = UserDAO {
            id: Some(id),
            name: name.clone(),
            email: None,
            password: password.clone(),
            avatar_url: avatar_url.clone(),
            updated_at: Some(Utc::now().naive_utc()),
        };

        let target_user = dsl::users.find(id);
        let count = diesel::update(target_user)
            .set(user_to_update)
            .execute(&self.conn)?;

        Ok(count)
    }

    pub fn delete(&self, id: u64) -> Result<usize, Error> {
        let target_user = dsl::users.find(id);
        // Consider also logical deletion
        let count = diesel::delete(target_user).execute(&self.conn)?;

        Ok(count)
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}
