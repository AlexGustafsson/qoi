.PHONY: native linux mac mac-arm download-examples clean

build: linux mac mac-arm

native:
	cargo build --release

linux:
		cargo build --release --target=x86_64-unknown-linux-musl

mac:
	cargo build --release --target=x86_64-apple-darwin

mac-arm:
	cargo build --release --target=aarch64-apple-darwin

download-examples:
	curl -o examples/examples.zip "https://qoiformat.org/qoi_test_images.zip"
	unzip -jd examples examples/examples.zip

clean:
	rm -rf target &>/dev/null || true
