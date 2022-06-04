.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream-updates
stream-updates:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml gravatar_updates

.PHONY: stream-gravatars
stream-gravatars:
	substreams run -e api-dev.streamingfast.io:443 substreams.yaml gravatars

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"
