// use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(from: &str, to: &str, reply_to: &str, subject: &str, body: &str) {
    // connect to smtp server
    // let creds = Credentials::new("user".to_string(), "pass".to_string());
    let mailer = SmtpTransport::builder_dangerous("127.0.0.1")
                    .port(1025)
                    .build();

    // build email
    let email = Message::builder()
        .from(from.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject.to_string())
        .body(body.to_string())
        .unwrap();

    // send email
    match mailer.send(&email) {
        Ok(_) => println!("mail sent"),
        Err(e) => panic!("no mail: {:?}", e),
    }
}
