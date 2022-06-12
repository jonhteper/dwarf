all: prepare build

prepare:
	mkdir bin || true
	cp assets/dwarf.desktop bin/dwarf.desktop
	cp assets/icon.png bin/icon.png
	cp assets/linux-install.sh bin/linux-install.sh
	cp assets/manual.txt bin/manual.txt
	cp LICENSE bin/LICENSE

build:
	cargo fmt
	cargo build --release
	cp target/release/dwarf bin/dwarf

dev: prepare
	cargo fmt
	cargo build
	cp target/debug/dwarf bin/dwarf
