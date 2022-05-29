# contact-rs

[![Build](https://github.com/chrisx8/contact-rs/actions/workflows/build_main.yml/badge.svg)](https://github.com/chrisx8/contact-rs/actions/workflows/build_main.yml)

A fast, simple, and memory-safe contact form processor, written in [Rust](https://www.rust-lang.org/) and powered by [Rocket](https://rocket.rs/).

## Configuration

Configuration is handled via environment variables.

Full example: [`.env.example`](.env.example)

| Environment variable   | Description                                                         | Default                                |
| ---------------------- | ------------------------------------------------------------------- | -------------------------------------- |
| `CORS_ALLOWED_ORIGINS` | CORS 'Access-Control-Allow-Origin' header                           | **Required**                           |
| `HCAPTCHA_SECRET`      | hCaptcha Secret Key                                                 | `None` (hCaptcha will not be verified) |
| `MAIL_FROM`            | "From" address for emails to site owner. Format: `mail@example.com` | **Required**                           |
| `MAIL_TO`              | "To" address for emails to site owner. Format: `mail@example.com`   | **Required**                           |
| `SMTP_HOST`            | SMTP server hostname                                                | **Required**                           |
| `SMTP_PORT`            | SMTP SSL port number                                                | **Required**                           |
| `SMTP_USERNAME`        | SMTP username.                                                      | *blank*                                |
| `SMTP_PASSWORD`        | SMTP password.                                                      | *blank*                                |

## Installation

### Docker

```bash
# This runs the contact-rs container on port 8000
# Make sure you have an env file (see Configuration section in README)
docker run -d -p 8000:8000 --env-file .env ghcr.io/chrisx8/contact-rs
```

### Pre-built binary

Pre-compiled binaries for Linux (x86_64) are available on the **Releases** page.

```bash
# Set Rocket environment variables
# Change ROCKET_ADDRESS and ROCKET_PORT as needed
export ROCKET_PROFILE=release \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000
# Make file executable
chmod +x contact-rs
# Launch
./contact-rs
```

### Build from source

See [BUILDING.md](BUILDING.md).

## Usage

### Request (form submission)

To submit a form, send a POST request with JSON (`application/json`) to `/contact`.

Sample request: `POST /contact`

```json
{
    "name": "My Name",
    "email": "person@example.com",
    "subject": "Subject line",
    "message": "Hello world!",
    "h_captcha_response": "10000000-aaaa-bbbb-cccc-000000000001"
}
```

Note the following:

- **All fields are required.**
- `name`, `subject`, and `message` must not be empty.
- `email` must contain a valid email address.
- If using hCaptcha, `h_captcha_response` must contain a valid hCaptcha Response.
- If not using hCaptcha (empty `$HCAPTCHA_SECRET`), `h_captcha_response` should be blank.
- How you implement the frontend is up to you. A reference JavaScript implementation is available [here](https://github.com/chrisx8/hugo-personal-site/blob/main/assets/js/contact.js).

### Response

Responses are in JSON (`application/json`) and follows this format:

```json
{
    "status": 201,
    "message": "Thanks for reaching out!"
}
```

Possible responses:

- HTTP 201: Submission has been processed successfully, and an email has been sent to `$MAIL_TO`.
- HTTP 400: Submission contains invalid input. Possible causes: some fields are not valid (see "Note the following" above), or hCaptcha response is invalid.
- HTTP 500: There's a server-side error. Possible causes: hCaptcha API is unreachable, or email failed to send.

## License

[GNU AGPLv3](LICENSE)

Copyright (c) 2022 Chris Xiao
