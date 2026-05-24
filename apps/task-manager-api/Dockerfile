# ========================================================
# STAGE 1: Builder
# ========================================================
FROM rust:1.95-slim AS builder

WORKDIR /workspace

# Docker layer caching for Cargo dependencies
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY apps/careerforge-api/Cargo.toml apps/careerforge-api/Cargo.toml
COPY apps/task-manager-api/Cargo.toml apps/task-manager-api/Cargo.toml
RUN mkdir -p apps/careerforge-api/src apps/task-manager-api/src \
    && echo "fn main() {}" > apps/careerforge-api/src/main.rs \
    && echo "fn main() {}" > apps/task-manager-api/src/main.rs \
    && cargo build --release -p task_manager_api \
    && rm -rf apps/careerforge-api/src apps/task-manager-api/src

# Copy real source code
COPY apps/task-manager-api/src apps/task-manager-api/src
RUN touch apps/task-manager-api/src/main.rs

# Build our release binary
RUN cargo build --release -p task_manager_api

# ========================================================
# STAGE 2: Runtime
# ========================================================
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled executable (name matches Cargo.toml package name)
COPY --from=builder /workspace/target/release/task_manager_api /app/task_manager_api

EXPOSE 3000

# Run our task manager server!
CMD ["./task_manager_api"]
