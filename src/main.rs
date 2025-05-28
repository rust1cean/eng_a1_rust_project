use axum::*;
use rand::prelude::*;
use serde::*;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world)).route("/get_cookie", get(get_cookie));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn hello_world() -> impl IntoResponse {
    "Hello, world!"
}

async fn get_cookie() -> Cookie {
    let wish = format!("Today is your day!");
    let flavor = Flavor::random_flavor();

    Cookie { wish, flavor }
}

#[derive(Debug, Serialize)]
struct Cookie {
    wish: String,
    flavor: Flavor
}

#[derive(Debug, Serialize)]
enum Flavor {
    Oatmeal,
    Chocolate,
    CottageCheese
}

impl Flavor {
    fn random_flavor() -> Flavor {
        let mut rng = rand::rng();

        match rng.random_range(0..=2) {
            0 => Flavor::Oatmeal,
            1 => Flavor::Chocolate,
            _ => Flavor::CottageCheese
        }
    }
}

