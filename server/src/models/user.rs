use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::models::connection;
use crate::models::error::{Error, Result};
use crate::schema::{users, users::dsl};

/// User representing `users` table.
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

/// User DAO using between models layer and RDB.
#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
struct UserDAO {
    id: Option<u64>,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    avatar_url: Option<String>,
    updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct ReCaptchaResponse {
    pub success: bool,
}

/// A core data repository for user.
pub struct UserRepository {
    conn: MysqlConnection,
}

#[automock]
pub trait UserRepositoryTrait {
    fn find_by_id(&self, id: u64) -> Result<User>;
    fn find_by_email(&self, email: &str) -> Result<User>;
    fn find_password_by_email(&self, email: &str) -> Result<String>;
    fn find_all(&self) -> Result<Vec<User>>;
    fn create(
        &self,
        name: &str,
        email: &str,
        password: &str,
        avatar_url: &Option<String>,
    ) -> Result<bool>;
    fn update(
        &self,
        id: u64,
        name: &Option<String>,
        password: &Option<String>,
        avatar_url: &Option<String>,
    ) -> Result<bool>;
    fn delete(&self, id: u64) -> Result<bool>;
}

impl UserRepository {
    /// Creates a new user repository.
    pub fn new() -> Self {
        Self {
            conn: connection::connect_rdb(),
        }
    }

    /// Finds a user by id.
    pub fn find_by_id(&self, id: u64) -> Result<User> {
        let user = dsl::users.find(id).get_result::<User>(&self.conn)?;
        Ok(user)
    }

    /// Finds a user by email.
    pub fn find_by_email(&self, email: &str) -> Result<User> {
        let user = dsl::users
            .filter(dsl::email.eq(email))
            .get_result::<User>(&self.conn)?;

        Ok(user)
    }

    /// Finds a password of the user specified by email.
    pub fn find_password_by_email(&self, email: &str) -> Result<String> {
        let password = dsl::users
            .select(dsl::password)
            .filter(dsl::email.eq(email))
            .get_result::<String>(&self.conn)?;

        Ok(password)
    }

    /// Finds all users.
    pub fn find_all(&self) -> Result<Vec<User>> {
        let user_list = dsl::users.load::<User>(&self.conn)?;
        Ok(user_list)
    }

    /// Creates a new user.
    pub fn create(
        &self,
        name: &str,
        email: &str,
        password: &str,
        avatar_url: &Option<String>,
    ) -> Result<bool> {
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

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }

    /// Updates a new user.
    pub fn update(
        &self,
        id: u64,
        name: &Option<String>,
        password: &Option<String>,
        avatar_url: &Option<String>,
    ) -> Result<bool> {
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

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }

    /// Deletes a user.
    pub fn delete(&self, id: u64) -> Result<bool> {
        let target_user = dsl::users.find(id);
        // Consider also logical deletion
        let count = diesel::delete(target_user).execute(&self.conn)?;

        if count > 0 {
            Ok(true)
        } else {
            Err(Error::QueryExecutionFailure)
        }
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}
