run-dev:
    RUST_LOG=none,eframe_template=debug cargo r
check:
    cargo fmt --check
    cargo clippy --all