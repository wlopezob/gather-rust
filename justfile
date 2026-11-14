check:
    cargo fmt --all --check
    cargo clippy --all-targets --locked -- -D warnings
    cargo nextest run --locked
    cargo test --doc --locked
    cargo deny check
