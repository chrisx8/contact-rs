use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

struct SMTPConfig<'r> {
    host: &'r str,     // smtp hostname
    port: u16,         // smtp port
    username: &'r str, // smtp username, blank = no auth
    password: &'r str, // smtp password, blank = no auth
}

pub struct Mail<'a> {
    pub from: &'a str,
    pub reply_to: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub body: &'a str,
}

fn get_mail_config() -> SMTPConfig<'static> {
    SMTPConfig {
        host: option_env!("SMTP_HOST").expect("$SMTP_HOST is not defined!"),
        port: option_env!("SMTP_PORT")
            .expect("$SMTP_PORT is not defined!")
            .parse()
            .expect("$SMTP_PORT is invalid!"),
        username: option_env!("SMTP_USERNAME").unwrap_or(""),
        password: option_env!("SMTP_PASSWORD").unwrap_or(""),
    }
}

pub fn send_email(m: &Mail) -> Result<(), Box<dyn Error>> {
    // TODO: Actually connect to SMTP and send emails
    let conf = get_mail_config();

    // connect to smtp server
    let mut mailer_builder = SmtpTransport::relay(conf.host)
        .unwrap()
        .port(conf.port);
    // authenticate if credentials exist
    if conf.username != "" && conf.password != "" {
        let creds = Credentials::new(conf.username.to_string(), conf.password.to_string());
        mailer_builder = mailer_builder.credentials(creds);
    }
    let mailer = mailer_builder.build();

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
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
