use serde::Deserialize;
use std::env;
use std::error::Error;

struct HCaptchaConfig {
    site_verify_url: &'static str,
    secret_key: String,
}

#[derive(Deserialize)]
struct HCaptchaResponse {
    success: bool,
}

/* Get HCaptcha config from environment variables
   Returns HCaptchaConfig struct (see above)
*/
fn get_hcaptcha_config() -> HCaptchaConfig {
    HCaptchaConfig {
        site_verify_url: "https://hcaptcha.com/siteverify",
        secret_key: env::var("HCAPTCHA_SECRET").unwrap_or(String::new()),
    }
}

/* Validate hCaptcha response with the hCaptcha API
   Returns (bool, Option<Box<Error>>):
    * `bool`: whether validation is successful
    * `Option<Box<Error>>`: any errors during validation (Some<Error> or None
       for no error)
*/
pub async fn validate_hcaptcha(respose: &str) -> (bool, Option<Box<dyn Error>>) {
    let config = get_hcaptcha_config();
    // skip validation if secret key is unset
    if config.secret_key == "" {
        return (true, None);
    }

    // request validation
    let body = [
        ("secret", config.secret_key),
        ("response", respose.to_owned()),
    ];
    let client = reqwest::Client::new();
    let resp = client
        .post(config.site_verify_url)
        .form(&body)
        .send()
        .await
        .unwrap();
    // check response & handle connection issues
    match resp.json::<HCaptchaResponse>().await {
        Ok(j) => return (j.success, None),
        Err(e) => return (false, Some(Box::new(e))),
    };
}
