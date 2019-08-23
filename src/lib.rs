use serde_json::value::Value;

pub trait FromForm {
    fn into_body(data: &Self, id: &str) -> Value;
    fn prefix() -> String;
}

pub mod id;

pub mod server;
