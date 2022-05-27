#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::Request;
mod hcaptcha;
mod mail;

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
struct StatusMsg<'r> {
    status: u16,
    message: &'r str,
}

/* Get MAIL_FROM and MAIL_TO from environment
   Returns tuple (MAIL_FROM, MAIL_TO)
*/
fn get_config() -> (&'static str, &'static str) {
    let from = option_env!("MAIL_FROM").expect("$MAIL_FROM is not defined!");
    let to = option_env!("MAIL_TO").expect("$MAIL_TO is not defined!");
    (from, to)
}

/* Generic HTTP error catcher
   Returns JSON StatusMsg (see above)
*/
#[catch(default)]
fn default_error(status: Status, _request: &Request) -> Json<StatusMsg<'static>> {
    Json::from(StatusMsg {
        status: status.code,
        message: status.reason().unwrap(),
    })
}

/* HTTP 400 (Captcha failure) catcher for /contact
   Returns Json StatusMsg (see above)
*/
#[catch(400)]
fn contact_captcha_error() -> Json<StatusMsg<'static>> {
    Json::from(StatusMsg {
        status: Status::BadRequest.code,
        message: "Captcha validation failed.",
    })
}

/* GET /
   Returns HTTP 200 "Hello world" as Json StatusMsg (see above)
*/
#[get("/")]
fn index() -> Json<StatusMsg<'static>> {
    Json::from(StatusMsg {
        status: Status::Ok.code,
        message: "Hello world!",
    })
}

/* POST /contact
   Returns Json StatusMsg (see above)
   HTTP 201 if successful
   HTTP 400 if Captcha validation fails
   HTTP 500 if server-side error occurs
*/
#[post("/contact", format = "json", data = "<message>")]
async fn contact(message: Json<Message<'_>>) -> Result<(Status, Json<StatusMsg<'_>>), Status> {
    // validate hcaptcha first
    let hcaptcha_result = hcaptcha::validate_hcaptcha(message.h_captcha_response).await;
    match hcaptcha_result {
        Ok(_) => {}
        Err(_e) => return Err(Status::BadRequest),
    };

    // send email
    let mail_from = format!("{} <{}>", message.name, "from@localhost");
    let mail_subject = format!("[Contact Form] {}", message.subject);
    let m = mail::Mail {
        from: mail_from.as_str(),
        reply_to: message.email,
        to: "to@localhost",
        subject: mail_subject.as_str(),
        body: message.message,
    };
    let mail_result = mail::send_email(&m);
    // handle potential email errors & respond
    match mail_result {
        Ok(_) => Ok((
            Status::Created,
            Json::from(StatusMsg {
                status: Status::Created.code,
                message: "Thanks for reaching out!",
            }),
        )),
        Err(e) => {
            eprintln!("***\n{:#?}\n***", e);
            Err(Status::InternalServerError)
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/contact", catchers![contact_captcha_error])
        .register("/", catchers![default_error])
        .mount("/", routes![index, contact])
}
