FROM rust:1.19.0
WORKDIR app
ADD . /app
EXPOSE 3000
RUN cargo build
ENTRYPOINT ["target/debug/rust-api"]