# Frame Transpiler Test Base Image (Segregated from Debugger)
# Ubuntu 22.04 LTS with all required toolchains for Frame transpiler testing
# NAMESPACE: frame-transpiler-* (segregated from frame-debugger-*)

FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

# Add transpiler-specific labels for identification
LABEL maintainer="Frame Transpiler Team"
LABEL namespace="frame-transpiler"
LABEL component="test-runner"
LABEL description="Frame Transpiler PRT Test Image - Segregated from Debugger"
LABEL version="1.0.0"

# Install system packages
RUN apt-get update && apt-get install -y \
    # Build essentials
    build-essential \
    cmake \
    pkg-config \
    # Version control
    git \
    # Network tools
    curl \
    wget \
    # Python 3.10+
    python3.10 \
    python3.10-dev \
    python3-pip \
    python3.10-venv \
    # Node.js setup dependencies
    ca-certificates \
    gnupg \
    lsb-release \
    # Utilities
    sudo \
    time \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 18.x LTS
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm@latest

# Install Rust stable
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust
ENV PATH=/opt/rust/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable \
    && rustup component add rustfmt clippy

# Create transpiler-specific user (not shared with debugger)
RUN useradd -m -s /bin/bash transpiler-tester \
    && echo 'transpiler-tester ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Create transpiler-specific mount points (segregated paths)
RUN mkdir -p /transpiler/framec /transpiler/fixtures /transpiler/results \
    && chown -R transpiler-tester:transpiler-tester /transpiler

# Install Python test dependencies
RUN pip3 install --no-cache-dir \
    pytest \
    pytest-timeout \
    pytest-json-report \
    junit-xml \
    py-compile-all

# Install TypeScript/Node test dependencies globally
RUN npm install -g \
    typescript@5.6.3 \
    @types/node@20.12.7 \
    mocha \
    tap

# Add transpiler test environment marker
ENV FRAME_TEST_ENV=transpiler
ENV FRAME_TEST_ISOLATION=true

# Verify installations
RUN python3 --version \
    && node --version \
    && npm --version \
    && rustc --version \
    && cargo --version \
    && tsc --version

# Switch to transpiler-specific user
USER transpiler-tester
WORKDIR /home/transpiler-tester

# Set up Rust for user
RUN rustup default stable

# Default command
CMD ["/bin/bash"]

# Health check with transpiler namespace
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD test "$FRAME_TEST_ENV" = "transpiler" && python3 -c "print('OK')" && node -e "console.log('OK')" && rustc --version || exit 1

# Volume mount points documentation (transpiler-specific)
# /transpiler/framec   - Mount the framec binary here
# /transpiler/fixtures - Mount test fixtures here  
# /transpiler/results  - Test results will be written here