use std::error::Error;

const HCAPTCHA_URL: &str = "https://hcaptcha.com/siteverify";

pub async fn validate_hcaptcha(respose: &str) -> Result<(), Box<dyn Error>> {
    let hcaptcha_secret: &str =
        option_env!("HCAPTCHA_SECRET").expect("$HCAPTCHA_SECRET is not defined!");

    let body = [("secret", hcaptcha_secret), ("response", respose)];
    let client = reqwest::Client::new();
    let resp = client.post(HCAPTCHA_URL).form(&body).send().await?;
    match resp.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
