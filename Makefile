compress: target/release/ajipsy
	@mkdir -p ./dest
	@tar -C target/release -cf dest/ajipsy.tar ajipsy
	@tar -rf dest/ajipsy.tar README.md
	@tar -rf dest/ajipsy.tar LICENSE
	@gzip -f dest/ajipsy.tar
