FROM ekidd/rust-musl-builder as builder
LABEL maintainer="Patrick Jusic <patrick.jusic@protonmail.com>"

WORKDIR /home/rust

COPY Cargo.toml Cargo.lock ./

RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN sudo touch src/main.rs

RUN cargo build --release


FROM alpine:latest

WORKDIR /root
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/plotter .
COPY --from=builder /home/rust/.env .

ENTRYPOINT ["./plotter"]
