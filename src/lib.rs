mod id;
mod serve;

use serde_json::Value;

pub use serve::serve;

pub trait Form {
    fn prefix() -> String;
    fn into_payload(&self, id: &str) -> Value;
    fn webhook() -> String;
}
