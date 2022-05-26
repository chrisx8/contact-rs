use std::error::Error;

const HCAPTCHA_URL: &str = "https://hcaptcha.com/siteverify";
const HCAPTCHA_SECRET: &str = "0x0000000000000000000000000000000000000000";

pub async fn validate_hcaptcha(respose: &str) -> Result<(), Box<dyn Error>> {
    let body = [("secret", HCAPTCHA_SECRET), ("response", respose)];
    let client = reqwest::Client::new();
    let resp = client.post(HCAPTCHA_URL)
                .form(&body)
                .send()
                .await?;
    match resp.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
