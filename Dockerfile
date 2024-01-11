FROM rust:latest

# Install dependencies
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> "$HOME/.bashrc"
RUN /bin/bash -c "source $HOME/.bashrc && rustup target add wasm32-unknown-unknown"
RUN /bin/bash -c "source $HOME/.bashrc && cargo install candid-extractor"

# Install DFX
ENV DFX_VERSION=0.15.0
RUN curl -fsSL https://sdk.dfinity.org/install.sh | sh -s -- --version=$DFX_VERSION
RUN echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"

# Set working directory
WORKDIR /app

# Entry point
CMD ["/bin/bash"]
