FROM rust:1.69-buster as builder

WORKDIR /source
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --bin uptimerobot-prometheus

###

FROM gcr.io/distroless/cc

COPY --from=builder /source/target/release/uptimerobot-prometheus /

CMD ["/uptimerobot-prometheus"]
