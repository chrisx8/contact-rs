#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
mod mail;
mod hcaptcha;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    name: &'r str,
    email: &'r str,
    subject: &'r str,
    message: &'r str,
    h_captcha_response: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response<'r> {
    status: &'r str,
    message: &'r str,
}

#[catch(404)]
fn not_found() -> &'static str {
    "404: Not found"
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/contact")]
fn contact_get() -> &'static str {
    "why are u here???"
}

#[post("/contact", format = "json", data = "<message>")]
async fn contact(message: Json<Message<'_>>) -> (Status, Json<Response<'_>>) {
    let mail_from = format!("{} <{}>", message.name, "from@localhost");
    let mail_subject = format!("[Contact Form] {}", message.subject);
    let m = mail::Mail {
        from: mail_from.as_str(),
        reply_to: message.email,
        to: "to@localhost",
        subject: mail_subject.as_str(),
        body: message.message,
    };

    let hcaptcha_result = hcaptcha::validate_hcaptcha(message.h_captcha_response).await;
    match hcaptcha_result {
        Ok(_) => {
            mail::send_email(&m);
            (Status::Created, Json::from(Response {status: "OK", message: "Thanks for reaching out!"}))
        },
        Err(_e) => {
            (Status::BadRequest, Json::from(Response {status: "Error", message: "Something went wrong"}))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, contact_get, contact])
}
