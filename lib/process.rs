use crate::{id, Form, IntoPayload};

use serde_json::Value;
use serde::Deserialize;
use std::{env, error::Error, fmt};
use ureq;

#[derive(Debug)]
pub struct ProcessError<'a>(&'a str);

pub fn process<'de, F>(payload: &'de str) -> Result<(), ProcessError<'de>>
where
    F: Deserialize<'de> + Form + IntoPayload<Value>,
{
    let json = match serde_json::from_str::<F>(payload) {
        Ok(json) => json,
        Err(_) => return Err(ProcessError("Error while parsing request")),
    };

    let webhook = env::var("FORM_WEBHOOK_URL")
        .expect("Required environment varialbe \"FORM_WEBHOOK_URL\" not present.");

    let response = ureq::post(&webhook)
        .set("Content-Type", "application/json")
        .send_json(json.into(&id::next(&F::prefix())));

    if response.ok() {
        Ok(())
    } else {
        Err(ProcessError("Error while sending webhook"))
    }
}

impl Error for ProcessError<'_> {}

impl<'a> Into<&'a str> for ProcessError<'a> {
    fn into(self) -> &'a str {
        self.0.into()
    }
}

impl<'a> Into<String> for ProcessError<'a> {
    fn into(self) -> String {
        self.0.to_owned()
    }
}

impl fmt::Display for ProcessError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
