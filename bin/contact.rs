mod server;

use api::contact;

fn main() {
    server::serve(contact);
}
