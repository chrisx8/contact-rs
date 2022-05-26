// use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct Mail<'a> {
    pub from: &'a str,
    pub reply_to: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub body: &'a str,
}

pub fn send_email(m: &Mail) {
    // connect to smtp server
    // let creds = Credentials::new("user".to_string(), "pass".to_string());
    let mailer = SmtpTransport::builder_dangerous("127.0.0.1")
                    .port(1025)
                    .build();

    // build email
    let email = Message::builder()
        .from(m.from.parse().unwrap())
        .reply_to(m.reply_to.parse().unwrap())
        .to(m.to.parse().unwrap())
        .subject(m.subject.to_string())
        .body(m.body.to_string())
        .unwrap();

    // send email
    match mailer.send(&email) {
        Ok(_) => println!("mail sent"),
        Err(e) => panic!("no mail: {:?}", e),
    }
}
