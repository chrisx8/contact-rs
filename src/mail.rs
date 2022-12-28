use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::error::Error;

struct SMTPConfig {
    unencrypted_localhost: bool, // use unencrypted smtp server at localhost
    host: String,                // smtp hostname
    port: u16,                   // smtp port
    username: String,            // smtp username, blank = no auth
    password: String,            // smtp password, blank = no auth
}

pub struct Mail {
    pub from: String,
    pub reply_to: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

/* Get SMTP config from environment variables
   Returns SMTPConfig struct (see above)
   Throws Error when required options are missing or malformatted
*/
fn get_mail_config() -> SMTPConfig {
    SMTPConfig {
        unencrypted_localhost: env::var("SMTP_UNENCRYPTED_LOCALHOST")
            .unwrap_or_else(|_| String::from("false"))
            .parse()
            .unwrap_or(false),
        host: env::var("SMTP_HOST").expect("$SMTP_HOST is not defined!"),
        port: env::var("SMTP_PORT")
            .expect("$SMTP_PORT is not defined!")
            .parse()
            .expect("$SMTP_PORT is invalid!"),
        username: env::var("SMTP_USERNAME").unwrap_or_default(),
        password: env::var("SMTP_PASSWORD").unwrap_or_default(),
    }
}

/* Pre-validate SMTP config before Rocket launch */
pub fn check_config() {
    get_mail_config();
}

/* Send an email
   Specify email content as a Mail struct (see above)
   Returns nothing if successful
   Throws Error if there's an issue
*/
pub fn send_email(m: &Mail) -> Result<(), Box<dyn Error>> {
    let conf = get_mail_config();

    // connect to smtp server
    let mut mailer_builder = SmtpTransport::relay(&conf.host).unwrap().port(conf.port);
    // use unencrypted smtp server at localhost, $SMTP_HOST is ignored
    if conf.unencrypted_localhost {
        mailer_builder = SmtpTransport::builder_dangerous("localhost").port(conf.port);
    }
    // authenticate if credentials exist
    if !conf.username.is_empty() && !conf.password.is_empty() {
        let creds = Credentials::new(conf.username, conf.password);
        mailer_builder = mailer_builder.credentials(creds);
    }
    let mailer = mailer_builder.build();

    // build email
    let email = Message::builder()
        .from(m.from.parse().unwrap())
        .reply_to(m.reply_to.parse().unwrap())
        .to(m.to.parse().unwrap())
        .subject(m.subject.to_owned())
        .body(m.body.to_owned())
        .unwrap();

    // send email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
