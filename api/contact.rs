use lazy_static::lazy_static;
use lib::{process, Form, IntoPayload};
use now_lambda::{error::NowError, Body, IntoResponse, Request};
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    #[serde(deserialize_with = "email_address_deserializer")]
    email: String,
    message: String,
}

impl Form for ContactForm {
    fn prefix() -> String {
        "CM".into()
    }
}

impl IntoPayload<Value> for ContactForm {
    fn into(self, id: &str) -> Value {
        json!({
            "embeds": [{
                "title": format!(
                    "Message {} from \"{}\" ({})",
                    id,
                    self.name,
                    self.email
                ),
                "description": self.message
            }]
        })
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

pub fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let body = match request.body() {
        Body::Text(body) => body,
        _ => return Err(NowError::new("Unknown payload")),
    };

    match process::<ContactForm>(body) {
        Ok(response) => Ok(response),
        Err(error) => return Err(NowError::new(error.into())),
    }
}
