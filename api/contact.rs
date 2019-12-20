use {
    lazy_static::lazy_static,
    lib::{process, Form, IntoPayload, Webhook},
    now_lambda::{error::NowError, Body, IntoResponse, Request},
    regex::Regex,
    serde::{Deserialize, Deserializer},
    serde_json::{json, Value},
    std::env,
};

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

impl Webhook for ContactForm {
    fn webhook() -> String {
        env::var("CONTACT_WEBHOOK_URL")
            .expect("Required environment varialbe \"CONTACT_WEBHOOK_URL\" not present.")
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
