build:
	cargo build

buildrel:
	cargo build --release

bench: buildrel
	hyperfine \
		-n rust-royal \
		-r 800 \
		-w 10 \
		"./target/release/royallist ~/Documents/Web-programes/next-blog"

benchcmp: buildrel
	hyperfine \
		-r 800 \
		-w 10 \
		-L TEST "./target/release/royallist,ls" \
		"{TEST} ~/Documents/Web-programes/next-blog"

install: buildrel
	doas mv ./target/release/royallist /usr/bin/royallist
