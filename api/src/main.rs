use std::sync::{Arc, Mutex};

use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};
use store::store::Store;

use crate::routes::{user::{sign_in, sign_up}, website::{create_website, get_website}};

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let s = Arc::new(Mutex::new(Store::new().unwrap()));

    let app = Route::new()
        .at("/website/:websiteId", get(get_website))
        .at("website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("user/signin", post(sign_in))
        .data(s)
        ;

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
