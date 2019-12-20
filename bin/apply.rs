mod server;

use api::apply;

fn main() {
    server::serve(apply);
}
