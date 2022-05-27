use std::error::Error;

struct HCaptchaConfig<'r> {
    site_verify_url: &'r str,
    secret_key: &'r str,
}

/* Get HCaptcha config from environment variables
   Returns HCaptchaConfig struct (see above)
*/
fn get_hcaptcha_config() -> HCaptchaConfig<'static> {
    HCaptchaConfig {
        site_verify_url: "https://hcaptcha.com/siteverify",
        secret_key: option_env!("HCAPTCHA_SECRET").unwrap_or(""),
    }
}

pub async fn validate_hcaptcha(respose: &str) -> Result<(), Box<dyn Error>> {
    let config = get_hcaptcha_config();

    // skip validation if secret key is unset
    if config.secret_key == "" {
        return Ok(());
    }

    let body = [("secret", config.secret_key), ("response", respose)];
    let client = reqwest::Client::new();
    let resp = client.post(config.site_verify_url).form(&body).send().await?;
    match resp.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
