# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/rvben/homewizard-p1-exporter/compare/v0.2.0...v0.2.1) - 2026-09-27

### Fixed

- **deps**: update rustls to 0.23.45 for RUSTSEC-2026-0285 ([f3b932a](https://github.com/rvben/homewizard-p1-exporter/commit/f3b932ac0888bee8680deb9b1e0a76e8e87c30ef))
- **make**: preserve recursive make semantics ([fb16549](https://github.com/rvben/homewizard-p1-exporter/commit/fb16549346a97f7c1e9058a533d21e39bdbeaf29))

### Performance

- **ci**: run independent checks concurrently ([d7572a6](https://github.com/rvben/homewizard-p1-exporter/commit/d7572a6139498ea1f11dbde86f4fe459e5001205))
- **ci**: align husker build toolchain ([2f19816](https://github.com/rvben/homewizard-p1-exporter/commit/2f198168062477fcc0dd71149b02ab9d8fce292c))

## [0.2.0](https://github.com/rvben/homewizard-p1-exporter/compare/v0.1.5...v0.2.0) - 2026-04-30

### Added
- 3-phase metrics: `homewizard_p1_active_power_l{2,3}_watts`, `homewizard_p1_active_voltage_l{1,2,3}_volts`, `homewizard_p1_active_current_l{2,3}_amperes`, `homewizard_p1_voltage_sag_l{2,3}_count_total`, `homewizard_p1_voltage_swell_l{2,3}_count_total`
- Tolerant deserialization: missing or `null` numeric/optional fields decode as zero/empty defaults instead of failing the whole response
- Parse errors now include the response body for easier diagnosis

### Changed (breaking)
- Renamed `homewizard_p1_voltage_sag_count_total` → `homewizard_p1_voltage_sag_l1_count_total`
- Renamed `homewizard_p1_voltage_swell_count_total` → `homewizard_p1_voltage_swell_l1_count_total`

  The previous names misleadingly implied an aggregate; they were always sourced from the L1 field. Single-phase users should switch to the L1 variant. To get an aggregate across phases, use `sum(homewizard_p1_voltage_sag_lN_count_total)`.

Thanks @dalax01 for the 3-phase contribution.

## [0.1.5] - 2025-01-23

### Added
- Dependabot configuration for automated dependency updates
- Enhanced Cargo.toml metadata for better crates.io discoverability

### Fixed
- Musl toolchain installation in release workflow for binary builds
- GitHub release creation with proper binary artifacts

## [0.1.4] - 2025-01-22

### Added
- OCI labels to Dockerfile for GitHub Container Registry integration
- Make release target for automated release process
- Multi-platform Docker builds (linux/amd64, linux/arm64, linux/arm/v7)

### Changed
- Standardized user naming in Docker container to 'exporter'
- Updated Docker build to use musl-based Alpine for better portability
- Improved release workflow to commit Cargo.lock automatically

### Fixed
- Cargo.lock commit issues in release pipeline
- Docker architecture mismatch in builds

## [0.1.3] - 2025-01-15

### Added
- Initial Prometheus exporter for HomeWizard P1 electricity meter
- Real-time electricity consumption monitoring
- Support for both active and total energy metrics
- Health check endpoint
- Docker support with multi-stage builds
- GitHub Actions CI/CD pipeline

### Features
- P1 protocol support for Dutch smart meters
- Active power consumption metrics
- Total energy consumption tracking
- Device status monitoring
- Configurable polling intervals
- TLS-enabled HTTP client
