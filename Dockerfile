# ====== BUILD ======
FROM docker.io/rust:1-alpine as build

COPY . /tmp

WORKDIR /tmp

RUN apk add --no-cache binutils musl-dev openssl-dev pkgconfig && \
    cargo install --path . --target=x86_64-unknown-linux-musl && \
    strip /usr/local/cargo/bin/contact-rs

# ====== RUNTIME ======
FROM docker.io/alpine:latest

ENV ROCKET_PROFILE="release" \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000

RUN apk add --no-cache openssl tzdata ca-certificates

COPY --from=build /usr/local/cargo/bin/contact-rs /usr/local/bin

EXPOSE 8000

CMD /usr/local/bin/contact-rs
