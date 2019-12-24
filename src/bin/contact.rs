use {
    env_logger,
    lazy_static::lazy_static,
    regex::Regex,
    rfgames_api_backend::{serve, Form},
    serde::{Deserialize, Deserializer},
    serde_json::{json, Value},
    std::{env, io},
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

    fn into_payload(&self, id: &str) -> Value {
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

fn main() -> io::Result<()> {
    env_logger::init();

    serve::<ContactForm>()
}
