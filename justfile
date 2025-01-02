lint:
    cargo fmt -- `find . -name "*.rs"`
    cargo clippy --all-targets --all-features
