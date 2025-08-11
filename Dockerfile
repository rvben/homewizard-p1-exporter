# Build stage - use the target platform's rust image
FROM rust:1.89-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application for the native platform
RUN cargo build --release --target $(rustc -vV | sed -n 's/host: //p') && \
    cp target/$(rustc -vV | sed -n 's/host: //p')/release/homewizard-p1-exporter /app/homewizard-p1-exporter

# Runtime stage
FROM alpine:3.22

# OCI labels for GitHub Container Registry
LABEL org.opencontainers.image.source=https://github.com/rvben/homewizard-p1-exporter
LABEL org.opencontainers.image.description="Prometheus exporter for HomeWizard P1 electricity meter"
LABEL org.opencontainers.image.licenses=MIT

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1000 exporter && \
    adduser -D -u 1000 -G exporter exporter

# Copy the binary from builder
COPY --from=builder /app/homewizard-p1-exporter /usr/local/bin/homewizard-p1-exporter

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

# Run the exporter
ENTRYPOINT ["/usr/local/bin/homewizard-p1-exporter"]