check:
    cargo fmt --all
    cargo clippy --all-targets --locked -- -D warnings
    cargo nextest run --locked
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked
    cargo test --doc --locked
    cargo deny check
