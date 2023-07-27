all: format build

format:
	cargo fmt

build:
	wasm-pack build --target web

lint:
	cargo clippy

deploy:
	mkdir ../dwarf-vue/src/utils/dwarf || true
	cp pkg/dwarf_wasm_bg.wasm ../dwarf-vue/src/utils/dwarf
	cp pkg/dwarf_wasm.js ../dwarf-vue/src/utils/dwarf
	cp pkg/dwarf_wasm.d.ts ../dwarf-vue/src/utils/dwarf