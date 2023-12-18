FROM messense/rust-musl-cross:x86_64-musl as builder

RUN apt-get update && apt-get install -y \
    git \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Clone the Rust project
RUN git clone --branch 0.8.3 https://github.com/s3m/s3m.git .
# RUN git clone --branch sandbox https://github.com/s3m/s3m.git .

RUN cargo build --release --locked --features "openssl/vendored"

From rust:latest

WORKDIR /app

RUN git clone --branch 0.8.3 https://github.com/s3m/s3m.git .

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/s3m /app/target/release/s3m

RUN cargo install cargo-generate-rpm

RUN strip -s /app/target/release/s3m

RUN cargo generate-rpm

ENTRYPOINT ["bash"]
