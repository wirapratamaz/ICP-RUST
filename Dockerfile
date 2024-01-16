FROM ubuntu:latest

# Install curl and other dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    software-properties-common \
    libunwind8

# Install Rust
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Install Candid Extractor
RUN cargo install candid-extractor

# Install DFX
RUN sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - 
RUN apt-get install -y nodejs

# Set the working directory to /app
WORKDIR /app
COPY . /app