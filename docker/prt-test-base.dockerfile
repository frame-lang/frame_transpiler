# PRT (Python, Rust, TypeScript) Test Base Image
# Ubuntu 22.04 LTS with all required toolchains for Frame testing

FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

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

# Create non-root user for test execution
RUN useradd -m -s /bin/bash testrunner \
    && echo 'testrunner ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Create mount points for volumes
RUN mkdir -p /framec /fixtures /results \
    && chown -R testrunner:testrunner /framec /fixtures /results

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

# Verify installations
RUN python3 --version \
    && node --version \
    && npm --version \
    && rustc --version \
    && cargo --version \
    && tsc --version

# Switch to non-root user
USER testrunner
WORKDIR /home/testrunner

# Set up Rust for user
RUN rustup default stable

# Default command
CMD ["/bin/bash"]

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD python3 -c "print('OK')" && node -e "console.log('OK')" && rustc --version || exit 1

# Labels
LABEL maintainer="Frame Development Team"
LABEL description="PRT Test Base Image with Python 3.10+, Node.js 18+, Rust stable"
LABEL version="1.0.0"

# Volume mount points documentation
# /framec   - Mount the framec binary here
# /fixtures - Mount test fixtures here
# /results  - Test results will be written here