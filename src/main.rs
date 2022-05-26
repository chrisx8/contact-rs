#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
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
struct JsonResponse<'r> {
    status: u16,
    message: &'r str,
}

#[catch(404)]
fn not_found() -> Json<JsonResponse<'static>> {
    Json::from(JsonResponse {
        status: Status::NotFound.code,
        message: "Not found",
    })
}

#[catch(500)]
fn server_error() -> Json<JsonResponse<'static>> {
    Json::from(JsonResponse {
        status: Status::InternalServerError.code,
        message: "Server is broken. Send help.",
    })
}

#[get("/")]
fn index() -> Json<JsonResponse<'static>> {
    Json::from(JsonResponse {
        status: Status::Ok.code,
        message: "Hello world!",
    })
}

#[post("/contact", format = "json", data = "<message>")]
async fn contact(message: Json<Message<'_>>) -> Result<(Status, Json<JsonResponse<'_>>), Status> {
    // validate hcaptcha first
    let hcaptcha_result = hcaptcha::validate_hcaptcha(message.h_captcha_response).await;
    match hcaptcha_result {
        Ok(_) => {}
        Err(_e) => {
            return Ok((
                Status::BadRequest,
                Json::from(JsonResponse {
                    status: Status::BadRequest.code,
                    message: "Captcha validation failed.",
                }),
            ))
        }
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
            Json::from(JsonResponse {
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
        .register("/", catchers![not_found, server_error])
        .mount("/", routes![index, contact])
}
