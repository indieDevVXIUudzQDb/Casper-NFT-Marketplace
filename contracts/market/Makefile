prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p market -p market-offer-purse --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/market.wasm 2>/dev/null | true
	wasm-strip target/wasm32-unknown-unknown/release/market-offer-purse.wasm 2>/dev/null | true

test-only:
	cargo test -p tests -- --nocapture

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm tests/wasm

build-contract-cep47 :
	cd ../casper-nft-cep47/ && make build-contract
	cp ../casper-nft-cep47/target/wasm32-unknown-unknown/release/*.wasm  tests/wasm

test: build-contract copy-wasm-file-to-test test-only

test-all: build-contract build-contract-cep47 copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf tests/wasm/*.wasm
