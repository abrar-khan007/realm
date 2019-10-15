mod db;
mod form;
mod in_;
pub mod point;
pub mod sql_types;
mod utils;

pub use db::{connection, rollback_if_required, DIESEL_POOL, db_test};
pub use form::{Form, FormErrors};
pub use in_::In;
pub use sql_types::{citext, CiString};
pub use utils::elapsed;

pub type Request = http::Request<Vec<u8>>;
pub type Result<T> = std::result::Result<T, failure::Error>;
pub type FResult<T> = Result<std::result::Result<T, FormErrors>>;

pub enum Language {
    English,
    Hindi,
}

impl Default for Language {
    fn default() -> Language {
        Language::English
    }
}

pub fn error_stack(err: &failure::Error) -> String {
    let mut b = "".to_string();
    for cause in err.iter_causes() {
        b.push_str(&format!("{}\n", cause));
    }
    b.push_str(&format!("{}\n", err.backtrace()));
    b
}

pub fn is_test() -> bool {
    std::env::args().any(|e| e == "--test")
}

pub fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(
        password,
        if is_test() {
            /* if we do not do this, tests are too slow */
            4
        } else {
            bcrypt::DEFAULT_COST
        },
    )
    .map_err(|e| e.into())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash).map_err(|e| e.into())
}
