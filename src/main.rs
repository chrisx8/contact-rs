#[macro_use] extern crate rocket;
use rocket::serde::Deserialize;
use rocket::serde::json::{ Json, Value, json };
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
fn contact(message: Json<Message<'_>>) -> Value {
    let from = format!("{} <{}>", message.name, "from@localhost");
    let from = from.as_str();
    let to = "to@localhost";
    let subject = format!("[Contact Form] {}", message.subject);
    let subject = subject.as_str();

    mail::send_email(from, to, message.email, subject, message.message);
    json!({
        "status": "ok",
        "message": "Thanks",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, contact_get, contact])
}
