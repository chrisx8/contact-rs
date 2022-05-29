#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::Request;
use std::env;
use validator::Validate;
mod cors;
mod hcaptcha;
mod mail;

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    #[validate(length(min = 1))]
    name: &'r str,
    #[validate(email)]
    email: &'r str,
    #[validate(length(min = 1))]
    subject: &'r str,
    #[validate(length(min = 1))]
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
fn get_config() -> (String, String) {
    let from = env::var("MAIL_FROM").expect("$MAIL_FROM is not defined!");
    let to = env::var("MAIL_TO").expect("$MAIL_TO is not defined!");
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

/* HTTP 400 catcher for /contact (Invalid Request or Bad Captcha)
   Returns Json StatusMsg (see above)
*/
#[catch(400)]
fn contact_invalid_req() -> Json<StatusMsg<'static>> {
    Json::from(StatusMsg {
        status: Status::BadRequest.code,
        message: "Invalid Request: Please check your input and try again.",
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
    // validate json request
    match message.validate() {
        Ok(_) => (),
        Err(_) => return Err(Status::BadRequest),
    };

    // get app config
    let (mail_from, mail_to) = get_config();

    // validate hcaptcha first
    let hcaptcha_result = hcaptcha::validate_hcaptcha(message.h_captcha_response).await;
    match hcaptcha_result {
        Ok(_) => {}
        Err(_) => return Err(Status::BadRequest),
    };

    // send email
    let from = format!("{} <{}>", message.name, mail_from);
    let subject = format!("[Contact Form] {}", message.subject);
    let m = mail::Mail {
        from: &from,
        reply_to: message.email,
        to: &mail_to,
        subject: &subject,
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
    // validate config here
    get_config();
    mail::check_config();

    rocket::build()
        .attach(cors::CORS)
        .register("/contact", catchers![contact_invalid_req])
        .register("/", catchers![default_error])
        .mount("/", routes![index, contact])
}
