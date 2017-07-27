extern crate hyper;
extern crate libc;
extern crate retry;
extern crate time;
extern crate uuid;
extern crate td_client;

use td_client::client::Client;

#[allow(unused_variables)]
fn main() {
    let client = Client::new("dummy_apikey");
    println!("Create a client");
}
