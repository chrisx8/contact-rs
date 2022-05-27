# contact-rs

A fast, simple, and memory-safe contact form processor, written in Rust

## Configuration

Configuration is handled via environment variables.

Full example: [`.env.example`](.env.example)

| Environment variable | Description                                                         | Default                                |
| -------------------- | ------------------------------------------------------------------- | -------------------------------------- |
| `HCAPTCHA_SECRET`    | hCaptcha Secret Key                                                 | `None` (hCaptcha will not be verified) |
| `MAIL_FROM`          | "From" address for emails to site owner. Format: `mail@example.com` | **Required**                           |
| `MAIL_TO`            | "To" address for emails to site owner. Format: `mail@example.com`   | **Required**                           |
| `SMTP_HOST`          | SMTP server hostname                                                | **Required**                           |
| `SMTP_PORT`          | SMTP SSL port number                                                | **Required**                           |
| `SMTP_USERNAME`      | SMTP username.                                                      | *blank*                                |
| `SMTP_PASSWORD`      | SMTP password.                                                      | *blank*                                |

## Installation

### Docker

TODO

### Pre-built binary

TODO

### Build from source

TODO

## License

[GNU AGPLv3](LICENSE)

Copyright (c) 2022 Chris Xiao
