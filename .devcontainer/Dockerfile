FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld postgresql-client \
   && apt-get autoremove -y && apt-get clean -y

USER vscode

# Install Rust CLI tools
RUN cargo install cargo-quickinstall \
   # && cargo quickinstall sea-orm-cli
   && cargo install \
      --git https://github.com/angelonfira/sea-orm \
      --branch add-json-binary-attribute \
      sea-orm-cli