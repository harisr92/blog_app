use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Queryable, Selectable, Insertable, Serialize, Deserialize, Identifiable, PartialEq,
)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    encrypted_password: Option<String>,
    password_salt: Option<String>,
}

impl User {
    pub fn build(
        first_name: Option<String>,
        last_name: Option<String>,
        email: String,
        password: String,
    ) -> Result<Self, String> {
        let mut rng = rand::thread_rng();
        let salt: [u8; 16] = rng.gen::<[u8; 16]>();
        let salt_str = match std::str::from_utf8(&salt) {
            Ok(slt) => String::from(slt),
            Err(e) => {
                println!("Error : {:?}", e);
                String::from("")
            }
        };
        println!("{:?}", salt_str);

        if let Ok(enc_pass) = bcrypt::hash_with_salt(password, 16, salt) {
            Ok(User {
                id: 0,
                first_name,
                last_name,
                email,
                encrypted_password: Some(enc_pass.to_string()),
                password_salt: Some(salt_str),
            })
        } else {
            Err("Could not hash password".to_string())
        }
    }

    pub fn compare_password(&self, password: String) -> bool {
        if let Some(enc_pass) = &self.encrypted_password {
            if let Ok(is_ok) = bcrypt::verify(password, enc_pass.as_str()) {
                is_ok
            } else {
                false
            }
        } else {
            false
        }
    }
}
