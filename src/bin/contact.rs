use lazy_static::lazy_static;
use regex::Regex;
use rfgames_backend_server::{server, FromForm};
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    #[serde(deserialize_with = "email_address_deserializer")]
    email: String,
    message: String,
}

impl FromForm for ContactForm {
    fn into_body(data: &Self, id: &str) -> Value {
        json!({
            "embeds": [{
                "title": format!(
                    "Message {} from \"{}\" ({})",
                    id,
                    data.name,
                    data.email
                ),
                "description": data.message
            }]
        })
    }

    fn prefix() -> String {
        "CM".into()
    }
}

fn email_address_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    lazy_static! {
        static ref EMAIL_REGEX: Regex =
            Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();
    }

    let account = String::deserialize(deserializer)?;

    if EMAIL_REGEX.is_match(&account) {
        Ok(account)
    } else {
        Err(serde::de::Error::custom("wrong email address format"))
    }
}

fn main() {
    env_logger::init();

    server::start::<ContactForm>();
}
