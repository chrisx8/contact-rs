// Reference: https://stackoverflow.com/a/64904947

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::env;

pub struct CORSHeaders;

#[rocket::async_trait]
impl Fairing for CORSHeaders {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let allowed_origins =
            env::var("CORS_ALLOWED_ORIGINS").expect("$CORS_ALLOWED_ORIGINS is not defined!");
        response.set_header(Header::new("Access-Control-Allow-Origin", allowed_origins));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, OPTIONS",
        ));
    }
}
