set shell := ["cmd", "/c"]

run example:
    cargo run --example {{example}}
    
run-fast example:
    set RUSTFLAGS=-C target-cpu=native && cargo run --release --example {{example}}