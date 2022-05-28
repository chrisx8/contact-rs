use std::env;
use std::error::Error;

struct HCaptchaConfig {
    site_verify_url: String,
    secret_key: String,
}

/* Get HCaptcha config from environment variables
   Returns HCaptchaConfig struct (see above)
*/
fn get_hcaptcha_config() -> HCaptchaConfig {
    HCaptchaConfig {
        site_verify_url: String::from("https://hcaptcha.com/siteverify"),
        secret_key: env::var("HCAPTCHA_SECRET").unwrap_or(String::new()),
    }
}

pub async fn validate_hcaptcha(respose: &str) -> Result<(), Box<dyn Error>> {
    let config = get_hcaptcha_config();

    // skip validation if secret key is unset
    if config.secret_key == "" {
        return Ok(());
    }

    let body = [
        ("secret", config.secret_key),
        ("response", respose.to_string()),
    ];
    let client = reqwest::Client::new();
    let resp = client
        .post(config.site_verify_url)
        .form(&body)
        .send()
        .await?;
    match resp.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
