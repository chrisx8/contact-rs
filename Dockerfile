# ====== BUILD ======
FROM docker.io/rust:1-alpine as build

COPY . /tmp

WORKDIR /tmp

RUN apk add --no-cache musl-dev openssl-dev pkgconfig && \
    cargo build --release

# ====== RUNTIME ======
FROM docker.io/alpine:latest

ENV ROCKET_PROFILE="release" \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000

RUN apk add --no-cache openssl tzdata ca-certificates

COPY --from=build /tmp/target/release/contact-rs .

EXPOSE 8000

CMD /contact-rs
