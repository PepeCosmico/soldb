build:
	cargo build-sbf --manifest-path ./programs/soldb_program/Cargo.toml

test: build
	export BPF_OUT_DIR=$(PWD)/target/deploy && cargo test --workspace --quiet

