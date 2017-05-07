extern crate mammut;
extern crate harvey_holt;
extern crate dotenv;

use mammut::{Data, Mastodon, Registration};
use mammut::status_builder::StatusBuilder;
use mammut::apps::{AppBuilder, Scope};

fn main() {
    dotenv::dotenv().ok();
    if ::std::env::var("CLIENT_ID").is_ok() {
        existing_app();
    } else {
       new_app();
    }
}

#[allow(dead_code)]
fn new_app() {
    let app = AppBuilder {
        client_name: "Harvey Holt",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scope::ReadWrite,
        website: None,
    };

    let mut registration = Registration::new(env("BASE"))
        .expect("Registration creation failed");
    registration.register(app).expect("Register failed");
    let url = registration.authorise().expect("Registration authorise failed");

    println!("Please visit {}, authorise, and enter the code it gives you:", url);

    let mut code = String::new();
    std::io::stdin().read_line(&mut code).expect("Reading code failed");

    let mastodon = registration.create_access_token(code).expect("Creating access token failed");

    println!("{:#?}", mastodon.data);
}

#[allow(dead_code)]
fn existing_app() {
    let data = Data {
        base: env("BASE"),
        client_id: env("CLIENT_ID"),
        client_secret: env("CLIENT_SECRET"),
        redirect: String::from("urn:ietf:wg:oauth:2.0:oob"),
        token: env("TOKEN")
    };

    let mastodon = Mastodon::from_data(data).expect("Could not create Mastodon instance from data");

    let status = harvey_holt::random_location();
    println!("Posting {}", status);

    let sb = StatusBuilder::new(status);
    println!("StatusBuilder = {:#?}", sb);
    mastodon.new_status(sb).expect("Could not post status");

    println!("Goodnight!");
}

fn env(s: &str) -> String {
    ::std::env::var(s).unwrap_or_else(|_| {
        panic!("must have `{}` defined", s)
    })
}
