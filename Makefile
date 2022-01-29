.PHONY: download-examples

download-examples:
	curl -o examples/examples.zip "https://qoiformat.org/qoi_test_images.zip"
	unzip -jd examples examples/examples.zip
