mod id;
mod process;

pub use process::process;

pub trait Form {
    fn prefix() -> String;
}

pub trait IntoPayload<T> {
    fn into(self, id: &str) -> T;
}

pub trait Webhook {
    fn webhook() -> String;
}
