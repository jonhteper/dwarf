all: prepare build

prepare:
	mkdir bin || true
	cp assets/dwarf.desktop bin/dwarf.desktop
	cp assets/icon.svg bin/icon.svg
	cp assets/linux-install.sh bin/linux-install.sh
	cp LICENSE bin/LICENSE
	
build:
	cargo fmt
	cargo build --release
	cp target/release/dwarf bin/dwarf
