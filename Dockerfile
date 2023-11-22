FROM messense/rust-musl-cross:x86_64-musl AS builder
ENV SQLX_OFFLINE=true
WORKDIR /pih-rs
# Copy the source code
COPY . .
# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /pih-rs/target/x86_64-unknown-linux-musl/release/pih-rs /pih-rs
ENTRYPOINT ["/pih-rs"]

EXPOSE 3000
