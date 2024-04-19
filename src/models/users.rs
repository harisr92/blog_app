use crate::schema::users;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    Identifiable,
    PartialEq,
    AsChangeset,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub encrypted_password: String,
    pub password_salt: String,
}

impl User {
    pub fn build(
        first_name: Option<String>,
        last_name: Option<String>,
        email: String,
        password: String,
    ) -> Self {
        let pass_and_salt = Self::hash_password(password);
        User {
            id: 0,
            first_name,
            last_name,
            email,
            encrypted_password: pass_and_salt.0,
            password_salt: pass_and_salt.1,
        }
    }

    pub async fn find_by_id(id: u64, mut db: Connection<crate::config::Db>) -> Option<Self> {
        let user = crate::schema::users::table
            .filter(users::id.eq(id))
            .first(&mut db)
            .await;

        match user {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }

    pub async fn find_by_email(
        email: String,
        mut db: Connection<crate::config::Db>,
    ) -> Option<Self> {
        let user = crate::schema::users::table
            .filter(users::email.eq(email))
            .first(&mut db)
            .await;

        match user {
            Ok(u) => Some(u),
            Err(_) => None,
        }
    }

    pub fn set_password(&mut self, password: String) {
        (self.encrypted_password, self.password_salt) = Self::hash_password(password);
    }

    pub fn compare_password(&self, password: String) -> bool {
        let parsed_hash: PasswordHash;
        if let Ok(res) = PasswordHash::new(&self.encrypted_password) {
            parsed_hash = res;
        } else {
            return false;
        };

        if let Ok(_) = Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            true
        } else {
            false
        }
    }

    fn hash_password(password: String) -> (String, String) {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();

        if let Ok(password_hash) = argon.hash_password(password.as_bytes(), &salt) {
            (password_hash.to_string(), salt.to_string())
        } else {
            panic!("Could not hash password");
        }
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(
        req: &'r rocket::request::Request<'_>,
    ) -> rocket::request::Outcome<User, Self::Error> {
        use rocket::http::Status;
        use rocket_db_pools::{diesel::prelude::*, Connection};

        let cookies = req.cookies();
        let user_id: Option<u64> = if let Some(blog_auth) = cookies.get_private("blog_auth") {
            serde_json::from_str(blog_auth.value()).unwrap()
        } else {
            None
        };

        match user_id {
            Some(id) => {
                let mut db: Connection<crate::config::Db> = req.guard().await.unwrap();

                if let Ok(user) = crate::schema::users::table
                    .filter(crate::schema::users::id.eq(id))
                    .first(&mut db)
                    .await
                {
                    rocket::request::Outcome::Success(user)
                } else {
                    rocket::request::Outcome::Error((Status::NotFound, ()))
                }
            }
            None => rocket::request::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::users::User;

    #[test]
    fn it_builds_new_user() {
        let user = User::build(
            Some("Bruce".to_string()),
            Some("Wayne".to_string()),
            "brucewayne.com".to_string(),
            "hello2world!".to_string(),
        );

        assert_eq!(user.id, 0);
        assert_eq!(user.first_name, Some("Bruce".to_string()));
        assert_eq!(user.last_name, Some("Wayne".to_string()));
        assert_eq!(user.email, "brucewayne.com".to_string());
    }
}
