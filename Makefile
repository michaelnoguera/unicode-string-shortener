.PHONY = all build-program build-website serve clean build-release build-program-release build-website-release

all: build-program build-website

build-program: program/src
	cd program && cargo build

build-program-release: program/src
	cd program && cargo build --release

build-website: website/src
	cd website && trunk build
	rsync -r --delete website/dist/ docs

build-website-release: website/src
	cd website && trunk build --release --public-url https://noguera.dev/unicode-string-shortener
	rsync -r --delete website/dist/ docs

build-release: clean build-program-release build-website-release

serve: build-website
	cd website && trunk serve

clean:
	cd program && cargo clean
	cd website && trunk clean