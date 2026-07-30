
default: run

dev_flags     := "--features bevy/dynamic_linking"
release_flags := "--release"

# Build the project (profile=[dev|release])
build profile="dev":
    cargo build {{ if profile == "release" { release_flags } else { dev_flags } }}

# Build and run the project (profile=[dev|release])
run profile="dev":
    cargo run {{ if profile == "release" { release_flags } else { dev_flags } }}

# Perform checks with clippy
check:
    cargo clippy --all-targets --features bevy/dynamic_linking

# Remove build artifacts
clean:
    cargo clean

