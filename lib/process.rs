use {
    crate::{id, Form, IntoPayload, Webhook},
    serde_json::Value,
    serde::Deserialize,
    std::{error::Error, fmt},
    ureq
};

#[derive(Debug)]
pub struct ProcessError<'a>(&'a str);

pub fn process<'de, F>(payload: &'de str) -> Result<(), ProcessError<'de>>
where
    F: Deserialize<'de> + Form + IntoPayload<Value> + Webhook,
{
    let json = match serde_json::from_str::<F>(payload) {
        Ok(json) => json,
        Err(_) => return Err(ProcessError("Error while parsing request")),
    };

    let webhook = F::webhook();

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
