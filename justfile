# Show this help message
help:
    just --list

# Format code
fmt:
    cargo fmt
    cargo clippy --fix --allow-staged --allow-dirty

