# ====== BUILD ======
FROM docker.io/rust:1-alpine as build

ENV RUST_TARGET=x86_64-unknown-linux-musl

COPY . /tmp

WORKDIR /tmp

RUN apk add --no-cache binutils make musl-dev perl && \
    cargo build --locked --release --target $RUST_TARGET && \
    strip target/$RUST_TARGET/release/contact-rs

# ====== RUNTIME ======
FROM docker.io/alpine:latest

ENV RUST_TARGET=x86_64-unknown-linux-musl \
    ROCKET_PROFILE=release \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000

RUN apk add --no-cache ca-certificates tzdata

COPY --from=build /tmp/target/$RUST_TARGET/release/contact-rs /usr/local/bin

EXPOSE 8000
USER nobody

CMD /usr/local/bin/contact-rs
