# Build stage
FROM rust:1.88 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:3.22

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Create a non-root user
RUN addgroup -g 1000 exporter && \
    adduser -D -u 1000 -G exporter exporter

# Copy the binary from builder
COPY --from=builder /app/target/release/homewizard-p1-exporter /usr/local/bin/homewizard-p1-exporter

# Change ownership
RUN chown exporter:exporter /usr/local/bin/homewizard-p1-exporter

# Switch to non-root user
USER exporter

# Expose metrics port
EXPOSE 9898

# Set default environment variables
ENV LOG_LEVEL=info
ENV POLL_INTERVAL=10
ENV METRICS_PORT=9898

ENTRYPOINT ["/usr/local/bin/homewizard-p1-exporter"]