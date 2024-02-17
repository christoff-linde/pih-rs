FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /pih-rs


FROM chef as planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /pih-rs/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl


# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /pih-rs/target/x86_64-unknown-linux-musl/release/pih-rs /pih-rs
ENTRYPOINT ["/pih-rs"]

EXPOSE 3000
