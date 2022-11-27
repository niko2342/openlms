use std::env;

use argon2::Config;
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use once_cell::sync::Lazy;
use rand::distributions::{Alphanumeric, DistString};
use thiserror::Error;

use crate::schema::users;

static HASH_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    use dotenvy::dotenv;
    dotenv().ok();

    let secret = env::var("HASH_SECRET").expect("secret not set");
    secret.as_bytes().to_vec()
});

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_salt: String,
    pub password_hash: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_salt: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password_hash: password.into(),
            password_salt: "".into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    Database(#[from] diesel::result::Error),
    #[error(transparent)]
    Hash(#[from] argon2::Error),
}

impl User {
    pub fn create(
        conn: &mut PgConnection,
        data: &mut NewUser,
    ) -> Result<Self, UserError> {
        let mut config = Config::default();
        config.secret = &HASH_SECRET;

        data.password_salt =
            Alphanumeric.sample_string(&mut rand::thread_rng(), 12);

        data.password_hash = argon2::hash_encoded(
            data.password_hash.as_bytes(),
            data.password_salt.as_bytes(),
            &config,
        )?;

        let user = insert_into(users::table)
            .values(&*data)
            .get_result::<User>(conn)?;

        Ok(user)
    }
}
