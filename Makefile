format:
	cargo fmt && cargo clippy

test:
	cargo test

commit-and-push:
	git add .
	git commit -m "Update parser"
	git push origin

