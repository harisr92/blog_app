use rocket::http::{Cookie, CookieJar};
use rocket::request::{self, FromRequest, Request};
use rocket_db_pools::diesel::MysqlPool;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);

#[derive(Debug)]
pub struct UserSession<'a> {
    pub user_id: Option<u64>,
    cookies: Option<&'a CookieJar<'a>>,
    is_signedin: bool,
}

impl UserSession<'_> {
    pub fn new<'a>() -> UserSession<'a> {
        UserSession {
            user_id: None,
            cookies: None,
            is_signedin: false,
        }
    }

    pub fn build<'a>(user_id: Option<u64>, cookies: &'a CookieJar<'a>) -> UserSession<'a> {
        let signed_in = match user_id {
            Some(_) => true,
            None => false,
        };

        UserSession {
            user_id,
            cookies: Some(cookies),
            is_signedin: signed_in,
        }
    }

    pub fn build_from<'a>(req: &'a Request) -> UserSession<'a> {
        let user_id: Option<u64> = if let Some(blog_auth) = req.cookies().get("blog_auth") {
            serde_json::from_str(blog_auth.value()).unwrap()
        } else {
            None
        };

        let is_signed_in = if let Some(_) = user_id { true } else { false };

        UserSession {
            user_id,
            cookies: Some(req.cookies()),
            is_signedin: is_signed_in,
        }
    }

    pub fn signin(&self, signed_user: crate::models::users::User) {
        if let Ok(serialized_user) = serde_json::to_string(&signed_user.id) {
            if let Some(auth_cookies) = self.cookies {
                let cookie = Cookie::build(("blog_auth", serialized_user));
                auth_cookies.add_private(cookie);
            } else {
                panic!("Couldn't find cookies");
            }
        }
    }

    pub fn signout(self, signed_user: crate::models::users::User) -> Result<bool, &'static str> {
        if self.user_id != Some(signed_user.id) {
            panic!("You cannot signout");
        }

        if let Some(cookies) = self.cookies {
            cookies.remove_private("blog_auth");
        }
        Ok(true)
    }

    pub fn is_signedin(&self) -> bool {
        self.is_signedin
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<UserSession<'r>, Self::Error> {
        let user: Option<crate::models::users::User> =
            if let request::Outcome::Success(user) = req.guard().await {
                Some(user)
            } else {
                None
            };

        match user {
            Some(u) => {
                let session = UserSession::build(Some(u.id), req.cookies());
                request::Outcome::Success(session)
            }
            None => {
                let session = UserSession::build(None, req.cookies());
                request::Outcome::Success(session)
            }
        }
    }
}
