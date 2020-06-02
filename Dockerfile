FROM rust:latest as builder
RUN apt-get update
RUN cd /tmp && USER=root cargo new --bin dt-instance-webserver
WORKDIR /tmp/dt-instance-webserver

# Build Rust skeleton project, caching dependencies, before building.
COPY Cargo.toml Cargo.lock ./
RUN touch build.rs && echo "fn main() {println!(\"cargo:rerun-if-changed=\\\"/tmp/dt-instance-webserver/build.rs\\\"\");}" >> build.rs
RUN cargo build --release

# Force the build.rs script to run by modifying it
RUN echo " " >> build.rs
COPY ./src ./src
RUN cargo build --release --verbose

# Push built release to slim container
FROM debian:buster-slim
COPY --from=builder /tmp/dt-instance-webserver/target/release/dt-instance-webserver /usr/local/bin/dt-instance-webserver
CMD ["dt-instance-webserver"]
