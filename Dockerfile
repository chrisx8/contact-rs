# ====== BUILD ======
FROM docker.io/rust:1-slim-bullseye as build

COPY . /tmp

WORKDIR /tmp

RUN cargo build --jobs "$(nproc)" --locked --release && \
    strip target/release/contact-rs

# ====== RUNTIME ======
FROM docker.io/debian:bullseye-slim

ENV ROCKET_PROFILE=release \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000

RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    apt-get clean && \
    rm -rf /var/cache/apt /var/lib/apt

COPY --from=build /tmp/target/release/contact-rs /usr/local/bin

EXPOSE 8000
USER nobody

CMD /usr/local/bin/contact-rs
